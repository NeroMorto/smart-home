use crate::reportable_trait::Reportable;

pub struct ReportCons<T, U>
where
    T: Reportable,
    U: Reportable,
{
    composable1: T,
    composable2: U,
}

pub struct ReportGenerator<T> {
    composable: T,
}
impl ReportGenerator<()> {
    pub fn new() -> Self {
        Self { composable: () }
    }
}

impl Default for ReportGenerator<()> {
    fn default() -> Self {
        Self {
            composable: Default::default(),
        }
    }
}

impl<T: Reportable> ReportGenerator<T> {
    pub fn with<U: Reportable>(self, reportable: U) -> ReportGenerator<ReportCons<T, U>> {
        ReportGenerator {
            composable: ReportCons {
                composable1: self.composable,
                composable2: reportable,
            },
        }
    }
    pub fn report(self) -> String {
        let report = self.composable.report();

        if report.is_empty() {
            return String::from("Nothing to remport");
        }
        report
    }
}

impl Reportable for () {
    fn report(&self) -> String {
        String::new()
    }
}

impl<T, U> Reportable for ReportCons<T, U>
where
    T: Reportable,
    U: Reportable,
{
    fn report(&self) -> String {
        format!(
            "{}\n{}",
            self.composable1.report(),
            self.composable2.report()
        )
    }
}
