use image::io::Reader as ImageReader;
use lopdf::{Document, Stream, Object};
use lopdf::dictionary;  // Import the dictionary macro
use std::path::Path;

pub fn convert_jpg_to_pdf(input_path: &Path, output_path: &Path) {
    let img = ImageReader::open(input_path).unwrap().decode().unwrap().to_rgb8();
    let (width, height) = img.dimensions();

    let mut doc = Document::with_version("1.5");

    // Prepare the image data and dictionary for the Stream
    let image_data = img.into_raw();
    let image_dict = dictionary! {
        "Width" => Object::Integer(width as i64),
        "Height" => Object::Integer(height as i64),
        "ColorSpace" => "DeviceRGB",
        "BitsPerComponent" => Object::Integer(8),
        "Filter" => "DCTDecode",
    };
    let image_stream = Stream::new(image_dict, image_data.clone());

    // Add the image stream to the document
    let image_stream_id = doc.add_object(Object::Stream(image_stream));

    // Create a page dictionary
    let page_dict = dictionary! {
        "Type" => "Page",
        "MediaBox" => Object::Array(vec![Object::Integer(0), Object::Integer(0), Object::Integer(width as i64), Object::Integer(height as i64)]),
        "Contents" => Object::Reference(image_stream_id),
    };
    

    // Add the page dictionary to the document
    let page_id = doc.add_object(page_dict);

    // Create a pages dictionary (Page tree root)
    let pages_dict = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => Object::Integer(1),
    };

    let page_dict = dictionary! {
    "Type" => "Page",
    "MediaBox" => Object::Array(vec![Object::Integer(0), Object::Integer(0), Object::Integer(width as i64), Object::Integer(height as i64)]),
    "Contents" => Object::Reference(image_stream_id),
};


    // Add the pages dictionary to the document
    let pages_id = doc.add_object(pages_dict);

    // Create a catalog dictionary
    let catalog_dict = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    

    // Add the catalog dictionary to the document
    let catalog_id = doc.add_object(catalog_dict);

    // Set the catalog as the root of the document
    doc.trailer.set("Root", catalog_id);

    // Save the document to a file
    doc.save(output_path).unwrap();
}
