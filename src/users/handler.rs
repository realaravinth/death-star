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

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use futures;

use super::create_new_user;
use super::{Creds, LoginRequestPayload, NewCreds};
use crate::errors::*;
use crate::pow::verify_pow;

pub async fn sign_up(
    session: Session,
    creds: web::Json<NewCreds>,
) -> ServiceResult<impl Responder> {
    let new_creds = creds.into_inner();
    let pow = &new_creds.pow;
    verify_pow(&session, &pow).await?;
    create_new_user(&new_creds.creds.username, &new_creds.creds.password).await?;
    Ok(HttpResponse::Ok()
        .set_header(actix_web::http::header::CONNECTION, "close")
        .finish())
}

pub async fn sign_in(creds: web::Json<LoginRequestPayload>) -> ServiceResult<impl Responder> {
    let response = super::utils::utils::verify(creds.into_inner()).await;
    if response {
        return Ok(HttpResponse::Ok().finish());
    } else {
        return Err(ServiceError::AuthorizationRequired);
    }
}

pub async fn sign_out(id: Identity) -> ServiceResult<impl Responder> {
    id.forget();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("You are successfully signed out"))
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(INDEX)
}

pub static INDEX: &'static str = "
<!DOCTYPE html>
<html>
  <head>
    <script>
      const submitForm = () => {
        const password = document.getElementById('password').value;
        postData(password).then(
        console.log('submit'));
      };
      async function postData(password) {

        const payload = {
            'password': password,
        };
        console.log(payload)
        const response = await fetch('/api/login', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(payload),
        });
        if (response.ok) {
          alert('Succes!')
        } else {
          alert('Authentication failed');
        }
      }
    </script>
  </head>
  <body>
      <p>Password:</p>
      <input type='password' id='password' value='password' name='password' /><br />
      <input type='button' value='SUBMIT' onclick='submitForm()'>
  </body>
</html>
";
