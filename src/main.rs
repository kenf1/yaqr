use image;
use bardecoder;

fn main(){
    //import image
    let img_path = "./Tests/QrCodes/ex1.jpg".to_string();
    let img = image::open(img_path).unwrap();

    //default decoder
    let decoder = bardecoder::default_decoder();

    //print results
    let results = decoder.decode(&img);
    for result in results{
        println!("{}",result.unwrap());
    }
}