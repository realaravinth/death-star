// Copyright (c) 2020 Aravinth T M <realaravinth@batsense.net>.
// See the COPYRIGHT file at the top-level directory of this
// distribution

//This program is free software; you can redistribute it and/or
//modify it under the terms of the GNU General Public License
//as published by the Free Software Foundation; either version 2
//of the License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.

//You should have received a copy of the GNU General Public License
//along with this program; if not, write to the Free Software
//Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

use actix_http::ResponseBuilder;
use actix_web::{error::ResponseError, http::header, http::StatusCode, HttpResponse};
use diesel::result::Error as DBError;
use failure::Fail;
use serde::{Deserialize, Serialize};

use std::convert::From;

#[derive(Debug, PartialEq, Fail)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[fail(display = "some characters are not permitted")] //405j
    CharError,
    #[fail(display = "username exists")] //405
    UsernameExists,
    #[fail(display = "invalid credentials")]
    AuthorizationRequired,
    #[fail(display = "internal error")] // 500
    InternalServerError,
    #[fail(display = "timeout")] //408
    Timeout,
    #[fail(display = "bad request")] //400
    BadRequest,
    #[fail(display = "Unable to connect to DB")]
    UnableToConnectToDb,
    #[fail(display = "PoW required, request not processed")]
    PoWRequired,
}

#[derive(Serialize, Deserialize)]
#[cfg(not(tarpaulin_include))]
struct ErrorToResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .json(ErrorToResponse {
                error: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::CharError => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::UsernameExists => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::AuthorizationRequired => StatusCode::UNAUTHORIZED,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            ServiceError::UnableToConnectToDb => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::PoWRequired => StatusCode::PAYMENT_REQUIRED,
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(_kind, _info) => ServiceError::BadRequest,
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<actix_http::Error> for ServiceError {
    fn from(error: actix_http::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<argon2::Error> for ServiceError {
    fn from(error: argon2::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
