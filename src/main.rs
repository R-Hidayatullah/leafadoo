#![feature(seek_stream_len)]
mod anet_archive;

fn main() {
    println!("Hello, world!");

    let index = 3;
    // let file_path = "Gw2.dat";
    // let gw2_dat = anet_archive::AnetArchive::load_from_file(file_path).unwrap();
    // println!("{:?}", gw2_dat.dat_header);
    // println!("{:?}", gw2_dat.mft_header);
    // println!("MFT Data count : {}", gw2_dat.mft_data.len());
    // println!("MFT Data : {:?}\n\n", gw2_dat.mft_data.get(index).unwrap());

    let file_path_2 = "Local.dat";
    let gw2_dat_2 = anet_archive::AnetArchive::load_from_file(file_path_2).unwrap();
    println!("{:?}", gw2_dat_2.dat_header);
    println!("{:?}", gw2_dat_2.mft_header);
    println!("MFT Data count : {}", gw2_dat_2.mft_data.len());
    println!("MFT Data : {:?}", gw2_dat_2.mft_data.get(index).unwrap());
    println!("MFT Data Index count : {}", gw2_dat_2.mft_index_data.len());

    //let mft_data = gw2_dat.get_mft_data(file_path, index).unwrap();
    //println!("Content : {:0X?}", mft_data);

    println!("MFT ID : {:?}", gw2_dat_2.mft_index_data.get(16));
    println!("MFT index count : {}", gw2_dat_2.mft_index_data.len());
}
