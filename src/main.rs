use image::{self,DynamicImage,GenericImageView};
use image::imageops::CatmullRom; //resize algorithm
use bardecoder;

fn main(){
    let img_path = "./Tests/QrCodes/ex2.png";

    //import image & resize if nec
    let qrcode = image_import(&img_path);
    let tidy = image_dimensions(qrcode);
    
    //print qr code contents
    image_decode(tidy);
}

//import image
fn image_import(image_path: &str) -> DynamicImage{
    let temp_img = image::open(image_path.to_string()).unwrap();
    return temp_img;
}

//determine image dimensions & resize if any >= 600 pixels
fn image_dimensions(image: DynamicImage) -> DynamicImage{
    //set check & resize cutoffs 
    let (orig_cutoff,new_dim) = (600,300);

    let (width,height) = image.dimensions();
    println!("Image dimensions are: {} pixels in width x {} pixels in height.",width,height);

    /*
        resize image if width >= 600 pixels, else return original
            - maintains orig aspect ratio
            - uses CatmullRom filter (https://docs.rs/image/latest/image/imageops/enum.FilterType.html)
    */
    if (width >= orig_cutoff) | (height >= orig_cutoff){
        println!("Image width and/or height >= {} pixels -> will be resized, maintaining original aspect ratio.",orig_cutoff);

        let new_image = image::DynamicImage::resize(&image,new_dim,new_dim,CatmullRom);
        println!("Resized image dimensions: {:?}",new_image.dimensions());
        
        return new_image;
    }else{
        return image;
    }
}

//decode image
fn image_decode(image: DynamicImage){
    //default decoder
    let decoder = bardecoder::default_decoder();

    //print results
    let results = decoder.decode(&image);
    for result in results{
        println!("QR Code content: \n {}",result.unwrap());
    }
}