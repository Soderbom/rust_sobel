use std::env;
use std::fs::File; 
use std::io::Read;
use std::io::Write;
use std::io::BufReader;
use std::io;

fn arr_to_hex(arr: &[u8]) -> i32 {
    let mut sum: i32 = 0;
    for (i, val) in arr.iter().rev().enumerate() {
        sum += i32::from(*val)<<((arr.len()-i-1)*8);
    }
    sum
}


fn read_file(filename: &str) -> io::Result<Vec<u8>> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}


fn hexdump(buffer: &Vec<u8>) {    
    for i in 0..13 {
        print!("{:0>4X}:  ", i * 16);
        for j in 0..16 {
            print!("{:0>2X} ", buffer[j + i*16]);
            // Spacing to make output easier to read
            if (j+1) % 4 == 0 {
                print!(" ");
            }
        }
        println!();
    }
}


fn convert_greyscale(buffer: &Vec<u8>, offset: usize) -> Vec<u8> {
    let mut greyscale_vec = buffer.to_vec();

    // Step three because BMP RGB colors in three bytes
    for i in (offset..buffer.len()).step_by(3) {
        for j in 0..3 {
            greyscale_vec[i + j] = buffer[i+1] as u8;
        }
    }
    greyscale_vec
}


fn write_file(buffer: Vec<u8>, output_file: String) -> io::Result<()> {
    let mut file = File::create(output_file)?;
    file.write_all(buffer.as_slice())?;
    Ok(())
}


fn sobel_filter(pixels: Vec<u8>, offset: i32, width: i32) -> Vec<u8> {
    let mut sobel_vec = pixels.to_vec();

    // Sobel filter horizontal and vertical
    let sobel_x: [[i32; 3];3] = [
        [-1, 0, 1],
        [-2, 0, 2],
        [-1, 0, 1]
    ];
    let sobel_y: [[i32; 3];3] = [
        [-1, -2, -1],
        [0, 0, 0],
        [1, 2, 1]
    ];
    
    let arr_length = pixels.len() as i32;
    for i in (offset+1..arr_length-3).step_by(3)  {
        let mut sum_x: i32 = 0;
        let mut sum_y: i32 = 0;
        for y in 0usize..3 {
            for x in 0usize..3 {
                let position: i32 = i + ((y as i32 - 1) * width * 3) + ((x as i32 - 1) * 3);

                // Check for out of bound
                if position > offset && position < (pixels.len() as i32) {
                    let position = position as usize;
                    sum_x += (pixels[position] as i32) * sobel_x[y][x];
                    sum_y += (pixels[position] as i32) * sobel_y[y][x];
                }
                
            }
        }

        let mut sum = ((sum_x * sum_x + sum_y * sum_y) as f64).sqrt();
        if sum > 255.0 {
            sum = 255.0;
        }
         
        for j in 0usize..3 {
            sobel_vec[i as usize+j] = sum.trunc() as u8;
        }
    }

    sobel_vec
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = format!("{}.bmp", &args[2]);

    let buffer = read_file(input_file).unwrap();
    // Grab metadata from the file header
    let offset = buffer[10] as usize;
    let width = arr_to_hex(&buffer[18..22]);
    let height = arr_to_hex(&buffer[22..26]);

    println!("Reading file: {}\nSaving output as: {}", input_file, output_file);

    println!("Width: {}\nHeight: {}", width, height);
    println!("Offset to pixels: {}\n", offset);

    hexdump(&buffer);
    print!("Converting to greyscale... ");
    let greyscale = convert_greyscale(&buffer, offset);
    println!("Done.");
    print!("Applying filter... ");
    let result = sobel_filter(greyscale, offset as i32, width as i32);
    println!("Done.");
    print!("Writing file... ");
    write_file(result, output_file).unwrap();
    println!("Done.");
    
    
}

