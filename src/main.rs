mod func;
mod func_tests;

fn main(){
    //local
    let img_local = "./tests/QrCodes/ex1.jpg";
    func::from_local(img_local);

    //remote
    let img_url = "https://github.com/piderman314/bardecoder/blob/master/tests/images/needs_alignment.jpg?raw=true";
    func::from_remote(img_url);

    //download image -> tests/QrCodes folder
    func::save_img(img_url,"./tests/QrCodes/Example.jpg",image::ImageFormat::Jpeg);
}