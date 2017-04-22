extern crate hyper;

use hyper::Client;
use hyper::header::{ContentType, UserAgent};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::Ok;
use std::env;
use std::io::Read;


fn main(){
  
  // curl -XPOST 'http://www.nickelback.com/service/emaillist'
  //        -H 'Host: www.nickelback.com'
  //        -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:51.0) Gecko/20100101 Firefox/51.0'
  //        -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8'
  //        -H 'Accept-Language: en-US,en;q=0.5'
  //        --compressed
  //        -H 'Referer: http://www.nickelback.com/service/emaillist'
  //        -H 'Connection: keep-alive'
  //        -H 'Upgrade-Insecure-Requests: 1'
  //        --data 'formName=emailListForm&email=<email>&country=US&tos=0&tos=1&proceed=Submit'
  
  
  let url = "http://www.nickelback.com/service/emaillist";
  let mut post_form: String = "formName=emailListForm&country=US&tos=0&tos=1&proceed=Submit&email=".to_owned();


  let email: String = match env::args().nth(1) {
    Some(email) => email.to_owned(),
    None => {
      println!("Usage: <email_adddress>");
      return;
    }
  };

  post_form.push_str(&email);
  

  let client = Client::new(); 
  let mut res = client.post(url)
                  .body(&post_form.to_string())
                  .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
                  .header(UserAgent("User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:51.0) Gecko/20100101 Firefox/51.0".to_string()))
                  .send()
                  .unwrap();

  assert_eq!(res.status, hyper::Ok);

  let mut s = String::new();
  res.read_to_string(&mut s).unwrap();
  
  //println!("{}", s); //DEBUG
  println!("{} has been added the world's best email list ;)", email);

}
