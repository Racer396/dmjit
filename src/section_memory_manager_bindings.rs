use std::ffi::{CStr, CString};
use libc::c_void;
use llvm_sys::execution_engine::{LLVMCreateSimpleMCJITMemoryManager, LLVMMCJITMemoryManagerRef};
use llvm_sys::prelude::LLVMBool;

enum LLVMSectionMemoryManagerOpaque {}
type LLVMSectionMemoryManagerRef = *mut LLVMSectionMemoryManagerOpaque;

#[allow(non_snake_case)]
extern "C" {
    fn bnLLVMCreateSectionMemoryManager() -> LLVMSectionMemoryManagerRef;
    fn bnLLVMDestroySectionMemoryManager(r: LLVMSectionMemoryManagerRef);

    fn bnLLVMSectionManagerAllocateCodeSection(
        Opaque: LLVMSectionMemoryManagerRef,
        Size: ::libc::uintptr_t,
        Alignment: ::libc::c_uint,
        SectionID: ::libc::c_uint,
        SectionName: *const ::libc::c_char,
    ) -> *mut u8;

    fn bnLLVMSectionManagerAllocateDataSection(
        Opaque: LLVMSectionMemoryManagerRef,
        Size: ::libc::uintptr_t,
        Alignment: ::libc::c_uint,
        SectionID: ::libc::c_uint,
        SectionName: *const ::libc::c_char,
        IsReadOnly: LLVMBool,
    ) -> *mut u8;

    fn bnLLVMSectionManagerFinalizeMemory(
        Opaque: LLVMSectionMemoryManagerRef,
        ErrMsg: *mut *mut ::libc::c_char
    ) -> LLVMBool;
}

#[derive(Debug)]
pub struct SectionMemoryManager {
    internal_ref: LLVMSectionMemoryManagerRef,
    pub sections: Vec<(*mut u8, CString, usize)>
}

impl SectionMemoryManager {
    pub fn new() -> Self {
        let internal_ref = unsafe { bnLLVMCreateSectionMemoryManager() };
        return Self {
            internal_ref,
            sections: vec![]
        }
    }

    extern "C" fn allocate_code_section(
        opaque: *mut c_void,
        size: ::libc::uintptr_t,
        alignment: ::libc::c_uint,
        section_id: ::libc::c_uint,
        section_name: *const ::libc::c_char
    ) -> *mut u8 {
        let manager = opaque as *mut SectionMemoryManager;
        let allocated = unsafe {
            bnLLVMSectionManagerAllocateCodeSection(
                (*manager).internal_ref,
                size,
                alignment,
                section_id,
                section_name
            )
        };
        unsafe { (*manager).sections.push((allocated, CString::from(CStr::from_ptr(section_name)), size)); }
        return allocated;
    }

    extern "C" fn allocate_data_section(
        opaque: *mut c_void,
        size: ::libc::uintptr_t,
        alignment: ::libc::c_uint,
        section_id: ::libc::c_uint,
        section_name: *const ::libc::c_char,
        is_read_only: LLVMBool
    ) -> *mut u8 {
        let manager = opaque as *mut SectionMemoryManager;
        let allocated = unsafe {
            bnLLVMSectionManagerAllocateDataSection(
                (*manager).internal_ref,
                size,
                alignment,
                section_id,
                section_name,
                is_read_only
            )
        };
        unsafe { (*manager).sections.push((allocated, CString::from(CStr::from_ptr(section_name)), size)); }
        return allocated;
    }

    extern "C" fn finalize_memory(
        opaque: *mut c_void,
        err_msg: *mut *mut ::libc::c_char
    ) -> LLVMBool {
        let manager = opaque as *mut SectionMemoryManager;
        return unsafe {
            bnLLVMSectionManagerFinalizeMemory((*manager).internal_ref, err_msg)
        }
    }

    extern "C" fn destroy(opaque: *mut c_void) {
        let manager = opaque as *mut SectionMemoryManager;
        unsafe { manager.drop_in_place() }
    }


    pub unsafe fn create_mcjit_memory_manager(manager: *mut SectionMemoryManager) -> LLVMMCJITMemoryManagerRef {
        LLVMCreateSimpleMCJITMemoryManager(
            manager as *mut c_void,
            Self::allocate_code_section,
            Self::allocate_data_section,
            Self::finalize_memory,
            Option::Some(Self::destroy)
        )
    }
}

impl Drop for SectionMemoryManager {
    fn drop(&mut self) {
        unsafe {
            bnLLVMDestroySectionMemoryManager(self.internal_ref);
            self.internal_ref = std::ptr::null_mut();
            self.sections.clear()
        }
    }
}

