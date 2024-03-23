use std::{borrow::BorrowMut, cell::RefCell, collections::BTreeMap, ops::{Deref, DerefMut}, rc::Rc, str::FromStr};

use candid::{candid_method, CandidType, Principal};
use ic_cdk::{caller, query, update};
use serde::{Deserialize, Serialize};
use types::Rcbytes;
mod types;
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
//File Descriptor
type FD = u128;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct ICPFile {
    id : u128,
    owner: String,
    name: String,
    data : Rcbytes
}

impl ICPFile {
    fn get_stat(&self) -> ICPFileStat {
        ICPFileStat { name: self.name.clone(), size: self.data.0.borrow().len(), id: self.id }
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum ICPFileError {
    Error(String),
    NotAuthorized,
    NotFound
}

#[derive(CandidType)]
pub struct ICPFileStat {
    name: String,
    size: usize,
    id : FD
}

thread_local! {
    pub static FILE_STORE : RefCell<BTreeMap<FD, ICPFile>> = RefCell::new(BTreeMap::new());
    pub static NEXT_FD : RefCell<u128> = RefCell::new(1);
    pub static USER_FILES : RefCell<BTreeMap<String, Vec<u128>>> = RefCell::new(BTreeMap::new());
}


#[update(guard = "not_anonymous")]
async fn create_file(mut file : ICPFile) -> Result<FD, ICPFileError> {
    let fd = get_id();
    file.id = fd;
    file.owner = caller().to_text();
    FILE_STORE.with_borrow_mut(|store| {
        store.insert(fd, file)
    });

    USER_FILES.with_borrow_mut(|store| {
        let vectr = store.get_mut(&caller().to_text());
        if vectr.is_none() {
            let v_store = vec![fd];
            store.insert(caller().to_text(), v_store);
        } else {
            let v_store = vectr.unwrap();
            v_store.push(fd);
        }
    });

    Ok(fd)
}

#[update(guard = "not_anonymous")]
async fn add_chunk(id : FD, chunk : Vec<u8>) -> Result<(), ICPFileError> {
    // checking file ownwership
     FILE_STORE.with_borrow_mut(|store| {
        let file_opt = store.get_mut(&id);
        if file_opt.is_none() {
            return Err(ICPFileError::NotFound);
        }
    
        let file_ref = file_opt.unwrap();
        let owner_principal = Principal::from_str(file_ref.owner.as_str()).unwrap();
        if caller() != owner_principal {
            return Err(ICPFileError::NotAuthorized);
        }

        let f = file_ref.data.0.as_ref();
        let mut v = f.borrow_mut();
        v.extend(chunk);
        Ok(())

    })
}

#[update(guard = "not_anonymous")]
async fn truncate_file(id : FD) -> Result<(), ICPFileError> {
    // checking file ownwership
    FILE_STORE.with_borrow_mut(|store| {
        let file_opt = store.get_mut(&id);
        if file_opt.is_none() {
            return Err(ICPFileError::NotFound);
        }
    
        let file_ref = file_opt.unwrap();
        let owner_principal = Principal::from_str(file_ref.owner.as_str()).unwrap();
        if caller() != owner_principal {
            return Err(ICPFileError::NotAuthorized);
        }

        let f = file_ref.data.0.as_ref();
        let mut v = f.borrow_mut();

        v.clear();
        Ok(())

    })
}

#[update(guard = "not_anonymous")]
async fn delete_file(id : FD) -> Result<u128, ICPFileError> {
    // checking file ownwership
    
    USER_FILES.with_borrow_mut(|store| {
        let user_store_opt = store.get_mut(&caller().to_text());
        if user_store_opt.is_some() {
            let vector = user_store_opt.unwrap();
            let indx = vector.iter().position(|x|{if *x == id {
                return true;
            } else {return false;}});

            if indx.is_some() {
                vector.remove(indx.unwrap());
            }
        };
    });

    FILE_STORE.with_borrow_mut(|store| {
        let file_opt = store.get_mut(&id);
        if file_opt.is_none() {
            return Err(ICPFileError::NotFound);
        }
    
        let file_ref = file_opt.unwrap();
        let owner_principal = Principal::from_str(file_ref.owner.as_str()).unwrap();
        if caller() != owner_principal {
            return Err(ICPFileError::NotAuthorized);
        }
    
        store.remove(&id);
        Ok(id)
    })

}

#[query(guard = "not_anonymous")]
async fn get_files() -> Vec<ICPFileStat> {
    let vecc_id = USER_FILES.with_borrow(|store| store.get(&caller().to_text()).cloned().unwrap());
    let mut vec_stat = vec![];
    for fd in vecc_id {
        let file_result = FILE_STORE.with_borrow(|store| {
            let file_ref_opt = store.get(&fd);
            if file_ref_opt.is_none() {
                return Err(())
            }

            let file_ref = file_ref_opt.unwrap();
            Ok(file_ref.get_stat())
        });

        if file_result.is_err() {
            continue;
        }

        vec_stat.push(file_result.unwrap());
        
    }

    vec_stat
}

#[query(guard = "not_anonymous")]
async fn get_file(id : FD) -> Result<ICPFile, ICPFileError> {
    FILE_STORE.with_borrow_mut(|store| {
        let file_opt = store.get_mut(&id).cloned();
        if file_opt.is_none() {
            return Err(ICPFileError::NotFound);
        }
    
        let file_ref = file_opt.unwrap();
        let owner_principal = Principal::from_str(file_ref.owner.as_str()).unwrap();
        if caller() != owner_principal {
            return Err(ICPFileError::NotAuthorized);
        }

        Ok(file_ref)
        
    })
}


fn not_anonymous() -> Result<(), String> {
    if caller() == Principal::anonymous() {
        return Err("You are anonymous".to_string());
    }

    Ok(())
}

fn get_id() -> u128 {
    
    NEXT_FD.with(|counter_ref| {
        let mut writer = counter_ref.borrow_mut();
        *writer += 1;
        *writer
    })
}

#[query]
#[candid_method(query)]
fn export_candid() -> String {
    ic_cdk::export_candid!();
    __export_service()
}
