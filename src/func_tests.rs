#![cfg(test)]

use crate::func;

//image exists
#[test]
fn local_import_pass(){
    _ = func::image_import("./tests/QrCodes/ex1.jpg");
}

//image does not exist
#[test]
#[should_panic]
fn local_import_fail(){
    _ = func::image_import("./tests");
}

//image at url
#[test]
fn url_import_pass(){
    _ = func::image_from_url("https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true");
}

//no image at url
#[test]
#[should_panic]
fn url_import_fail(){
    let res = func::image_from_url("https://github.com/");
    res.unwrap();
}

//test local wrapper function
#[test]
fn from_local_pass(){
    _ = func::image_import("./tests/QrCodes/ex1.jpg");
}

//test local wrapper function
#[test]
#[should_panic]
fn from_local_fail(){
    _ = func::image_import("./tests");
}

//test url wrapper function
#[test]
fn from_url_pass(){
    _ = func::image_from_url("https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true");
}

//test local wrapper function
#[test]
#[should_panic]
fn from_url_fail(){
    let res = func::image_from_url("https://github.com/");
    res.unwrap();
}

//test save_img function
#[test]
fn save_img_pass(){
    let img_url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";
    func::save_img(img_url,"./tests/QrCodes/Example.jpg",image::ImageFormat::Jpeg);
}

#[test]
#[should_panic]
fn save_img_fail(){
    let img_url = "https://github.com/";
    func::save_img(img_url,"./tests/QrCodes/Example.jpg",image::ImageFormat::Jpeg);
}

#[test]
#[should_panic]
fn save_img_fail1(){
    let img_url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";
    func::save_img(img_url,"./tests",image::ImageFormat::Jpeg);
}