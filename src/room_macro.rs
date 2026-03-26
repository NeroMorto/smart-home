/// Creates a [`Room`] containing the ['Devices'].
///
/// - Create a [`Room`] with no devices:
/// ```
/// use smart_home_lib::*;
/// let room = room! {};
/// ```
///
/// - Create a [`Room`] containing a given mapping names to raw devices:
///
/// ```
/// use smart_home_lib::*;
/// use smart_home_lib::device::*;
/// use smart_home_lib::device::thermometer::Thermometer;
/// use smart_home_lib::device::static_thermometer::StaticThermometer;
/// use smart_home_lib::device::static_electrical_socket::StaticElectricalSocket;
///
/// let room = room! {
///     "Device 1" => Thermometer::new(Box::new(StaticThermometer::new(32.))),
///     "Device 2" => ElectricalSocket::new(Box::new(StaticElectricalSocket::new(220., false.into()))),
/// };
///
/// let device_1 = room.get_device("Device 1").unwrap();
/// assert!(matches!(device_1, Device::Thermometer(_)));
///
/// let device_2 = room.get_device("Device 2").unwrap();
/// assert!(matches!(device_2, Device::ElectricalSocket(_)));
/// ```
///
#[macro_export]
macro_rules! room {
    { $( $device_name:expr => $device:expr ), * $(,)? } => {
        {
            $crate::room::Room::new(vec![$(($device_name, Device::from($device))), *])
        }
    }
}
