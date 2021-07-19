fn main() {
    windows::build!(
        Windows::Win32::System::Diagnostics::Debug::{
            GetLastError,
        },
        Windows::Win32::System::SystemServices::{GENERIC_READ, GENERIC_ALL},
        Windows::Win32::System::Services:: {
            StartServiceA,
            QueryServiceStatusEx,
            QueryServiceStatus,
            OpenServiceA,
            OpenSCManagerA,
            EnumDependentServicesA,
            ControlServiceExW,
            CloseServiceHandle,
            ChangeServiceConfigW,
            SERVICE_STATUS_PROCESS,
            SC_STATUS_TYPE,
            ENUM_SERVICE_STATUSA,
            SERVICE_CONTROL_STOP,
            SERVICE_STATUS_PROCESS, 
            SERVICE_ENUMERATE_DEPENDENTS, 
            SERVICE_CONTROL_STATUS_REASON_INFO,
            SERVICE_CONTROL_STATUS_REASON_PARAMSW, 
            SERVICE_QUERY_STATUS,
            SERVICE_START_TYPE, 
        },
    )
}