mod devices;
mod rooms;

mod report;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{App, HttpServer, middleware::Logger, web};
use smart_home_lib::{
    SmartHome,
    device::{
        Device, ElectricalSocket, static_electrical_socket::StaticElectricalSocket,
        static_thermometer::StaticThermometer, thermometer::Thermometer,
    },
};

struct AppState {
    smart_home: Mutex<SmartHome>,
    br: Arc<BackendRegistry>,
}

type DeviceFactory = Box<dyn Fn() -> Result<Device, String> + Send + Sync>;

pub struct BackendRegistry {
    factories: HashMap<String, DeviceFactory>,
}

impl Default for BackendRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BackendRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, device_type: String, factory: F)
    where
        F: Fn() -> Result<Device, String> + Send + Sync + 'static,
    {
        self.factories.insert(device_type, Box::new(factory));
    }

    pub fn create_device(&self, device_type: &str) -> Result<Device, String> {
        let factory = self
            .factories
            .get(device_type)
            .ok_or_else(|| format!("Unknown device type: {}", device_type))?;

        factory()
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(devices::routes())
        .service(rooms::routes())
        .service(report::routes());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut registry = BackendRegistry::new();

    registry.register("thermometer".to_string(), || {
        let backend = Box::new(StaticThermometer::new(rand::random_range(10.0..40.0)));
        Ok(Device::Thermometer(Thermometer::new(backend)))
    });
    registry.register("socket".into(), || {
        let backend = Box::new(StaticElectricalSocket::new(
            rand::random_range(120.0..220.0),
            rand::random::<bool>().into(),
        ));
        Ok(Device::ElectricalSocket(ElectricalSocket::new(backend)))
    });
    let rg = Arc::new(registry);

    let state = web::Data::new(AppState {
        smart_home: Mutex::new(SmartHome::new(vec![])),
        br: rg,
    });
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .configure(configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_http::StatusCode;
    use actix_web::{App, test, web};
    use smart_home_lib::room::Room;
    use std::sync::{Arc, Mutex};

    async fn create_test_app() -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        let mut registry = BackendRegistry::new();
        registry.register("thermometer".to_string(), || {
            let backend = Box::new(StaticThermometer::new(25.5));
            Ok(Device::Thermometer(Thermometer::new(backend)))
        });
        registry.register("socket".into(), || {
            let backend = Box::new(StaticElectricalSocket::new(220.0, true.into()));
            Ok(Device::ElectricalSocket(ElectricalSocket::new(backend)))
        });

        let backend = Box::new(StaticElectricalSocket::new(110.0, true.into()));
        let device = Device::ElectricalSocket(ElectricalSocket::new(backend));

        let state = web::Data::new(AppState {
            smart_home: Mutex::new(SmartHome::new(vec![(
                "Kitchen",
                Room::new(vec![("kitchen-socket", device)]),
            )])),
            br: Arc::new(registry),
        });

        test::init_service(App::new().app_data(state.clone()).configure(configure)).await
    }

    #[actix_web::test]
    async fn test_get_rooms_success() {
        let app = create_test_app().await;

        let req = test::TestRequest::get().uri("/rooms").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_add_room_and_verify() {
        let app = create_test_app().await;

        let post_req = test::TestRequest::post()
            .uri("/rooms")
            .set_json(serde_json::json!({ "room_name": "Bedroom" }))
            .to_request();

        let post_resp = test::call_service(&app, post_req).await;
        assert_eq!(post_resp.status(), StatusCode::CREATED);

        let get_req = test::TestRequest::get().uri("/rooms").to_request();

        let get_resp = test::call_service(&app, get_req).await;
        assert_eq!(get_resp.status(), StatusCode::OK);

        let body = test::read_body(get_resp).await;
        let body_str = String::from_utf8_lossy(&body);

        assert!(body_str.contains("Bedroom"));
    }

    #[actix_web::test]
    async fn test_get_non_existent_room() {
        let app = create_test_app().await;

        let req = test::TestRequest::get().uri("/rooms/Bathroom").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_delete_room() {
        let app = create_test_app().await;
        let get_req = test::TestRequest::get().uri("/rooms/Kitchen").to_request();
        let get_resp = test::call_service(&app, get_req).await;

        assert_eq!(get_resp.status(), StatusCode::OK);

        let req = test::TestRequest::delete()
            .uri("/rooms/Kitchen")
            .to_request();
        let delete_resp = test::call_service(&app, req).await;
        assert_eq!(delete_resp.status(), StatusCode::NO_CONTENT);

        let get_req = test::TestRequest::get().uri("/rooms/Kitchen").to_request();
        let get_resp = test::call_service(&app, get_req).await;

        assert_eq!(get_resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_devices_success() {
        let app = create_test_app().await;

        let req = test::TestRequest::get()
            .uri("/rooms/Kitchen/devices")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_add_device_success() {
        let app = create_test_app().await;

        let req = test::TestRequest::post()
            .uri("/rooms/Kitchen/devices")
            .set_json(
                serde_json::json!({ "name": "Kitchen thermometer", "device_type":"Thermometer" }),
            )
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_get_device_detail_success() {
        let app = create_test_app().await;
        let req = test::TestRequest::get()
            .uri("/rooms/Kitchen/devices/kitchen-socket")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_non_existent_device() {
        let app = create_test_app().await;
        let req = test::TestRequest::get()
            .uri("/rooms/Kitchen/devices/some-device")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_delete_device() {
        let app = create_test_app().await;

        let req = test::TestRequest::get()
            .uri("/rooms/Kitchen/devices/kitchen-socket")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = test::TestRequest::delete()
            .uri("/rooms/Kitchen/devices/kitchen-socket")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let req = test::TestRequest::get()
            .uri("/rooms/Kitchen/devices/kitchen-socket")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
