use std::ptr::null_mut;
use bindings::{
    Windows::Win32::System::Diagnostics::Debug::{
        GetLastError,
    },
    Windows::Win32::Security::SC_HANDLE,
    Windows::Win32::System::Services::{
    QueryServiceStatusEx,
    OpenServiceA,
    OpenSCManagerA,
    CloseServiceHandle,
    StartServiceA,
    SERVICE_STATUS_PROCESS,
    SC_STATUS_PROCESS_INFO,
    }, 
    Windows::Win32::System::SystemServices::{GENERIC_ALL}};
use windows::HRESULT;
fn main() -> Result<(), HRESULT>{
    unsafe {
        let scm = SafeScMgrHandle::new();
        let service = SafeSvcHandle::new("Fax", scm);

        let mut buffer_size = 0;

        QueryServiceStatusEx(
            service.svc_handle,
            SC_STATUS_PROCESS_INFO,
            std::ptr::null_mut(),
            0,
            &mut buffer_size,
        );

        let mut buffer = Vec::new();
        buffer.resize(buffer_size as _, 0);

        if !QueryServiceStatusEx(
            service.svc_handle,
            SC_STATUS_PROCESS_INFO,
            buffer.as_mut_ptr(),
            buffer_size,
            &mut buffer_size,
        )
        .as_bool()
        {
            //println!("{:#?}", HRESULT::from_thread());
            return Err(HRESULT::from_thread());
        }

        let info = buffer.as_ptr() as *const SERVICE_STATUS_PROCESS;
        println!("{:#?}", *info);

        // let success = StartServiceA(service, 0, null_mut()).as_bool();

        // if !success{
        //     println!("{:#?}", GetLastError());
        //     return Err(String::from("Service didn't start...."))
        // }

        // println!("Service started successfully!");

        Ok(())
    }
}

pub struct SafeScMgrHandle {
    pub sc_manager_handle: SC_HANDLE,
}

impl  SafeScMgrHandle{
    pub fn new() -> Self{
        Self {
            sc_manager_handle: unsafe { OpenSCManagerA(None, None, GENERIC_ALL)},     
        }
    }
}

impl  Drop for SafeScMgrHandle {
    fn drop(&mut self) {
        if !self.sc_manager_handle.is_null() {
            unsafe { CloseServiceHandle(self.sc_manager_handle); }
        }
    }
}

pub struct SafeSvcHandle {
    pub svc_handle: SC_HANDLE,
}

impl SafeSvcHandle {
    pub fn new(svc_name:&str, sc_manager_handle:SafeScMgrHandle) -> Self {       
    
                Self {
                    svc_handle: unsafe {OpenServiceA(sc_manager_handle.sc_manager_handle, svc_name, GENERIC_ALL)},
                } 
    }
}

impl Drop for SafeSvcHandle {
    fn drop(&mut self) {
        if !self.svc_handle.is_null() {
            unsafe { CloseServiceHandle(self.svc_handle);}
        }
    }
}
