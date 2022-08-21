use pdf::file::File;


macro_rules! file_path {
    ( $subdir:expr ) => { concat!("../files/", $subdir) }
}



#[test]
fn count_annotations() {
    let pdf = File::<Vec<u8>>::open(file_path!("AnnotationDemo.pdf")).unwrap();
    assert_eq!(pdf.pages().count(), 1);

    let page = pdf.pages().next().unwrap().unwrap();
    if let Some(annots) = &page.annots {
        assert_eq!(annots.len(), 33);
    } else {
        panic!("Page should have annotations");
    }
}


