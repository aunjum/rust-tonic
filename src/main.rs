use tonic::{transport::Server, Request, Response, Status};

use bookstore::bookstore_server::{Bookstore, BookstoreServer};
use bookstore::{GetBookRequest, GetBookResponse};

use user::user_server::{User, UserServer};
use user::{GetUserRequest, GetUserResponse};

mod bookstore {
    include!("bookstore.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("greeter_descriptor");
}

mod user {
    include!("user.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("greeter_descriptor");
}


#[derive(Default)]
pub struct BookStoreImpl {}

#[tonic::async_trait]
impl Bookstore for BookStoreImpl {
    async fn get_book(
        &self,
        request: Request<GetBookRequest>,
    ) -> Result<Response<GetBookResponse>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let response = GetBookResponse {
            id: request.into_inner().id,
            author: "Peter".to_owned(),
            name: "Zero to One".to_owned(),
            year: 2014,
        };
        Ok(Response::new(response))
    }
}

#[derive(Default)]

pub struct UserImpl {}

#[tonic::async_trait]
impl User for UserImpl {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let response = GetUserResponse {
            id: request.into_inner().id,
            first_name: "Tanvir".to_owned(),
            last_name: "Aunjum".to_owned(),
            nick_name: "Sunny".to_owned(),
            gender: "Male".to_owned(),
            age: 27,
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let bookstore = BookStoreImpl::default();
    let user = UserImpl::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(&[bookstore::FILE_DESCRIPTOR_SET, user::FILE_DESCRIPTOR_SET].concat())
        .build()
        .unwrap();

    println!("Bookstore, User server listening on {}", addr);

    Server::builder()
        .add_service(BookstoreServer::new(bookstore))
        .add_service(UserServer::new(user))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}