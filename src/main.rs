use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Entry, ListBox, Button, Orientation};

fn create_html_file_if_not_exists(path: &str, content: &str) -> io::Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}

fn copy_file(source: &str, destination: &str) -> io::Result<()> {
    let html_content: &str = r###"<!DOCTYPE html>
<html>
<head>
<META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">
<TITLE>Lesezeichen-Menü</TITLE>
<style type="text/css">
    html { background-color: #3e3d39; color: #dfdbd2; font-family: arial; }
    body { margin: 0px 15px; }
    a:link, a:active, a:visited { text-decoration: none; color: #dfdbd2; }
    a:hover { font-weight: bold; color: white; text-shadow: 1px 1px 5px #dfdbd2; }
    H3 { background-color: #464540; }
    DD { color: grey; }
</style>
</head>
<body>
<H1>&nbsp;Lesezeichen-Menü</H1>
<div id="navibereich">
<ul><!--Navibereich-->
<!--#Example-->    <li><a href="#Example111">Example</a></li>
</ul>
</div>
<div id="textbereich">
<DL><!--Textbereich-->
    <p><a name="Example111"></a><br />
    <DT><H3>&nbsp;Example</H3>
    <DL><p><!--Example-->
    </DL>
</DL>
</div>
</body>
</html>
"###;
    create_html_file_if_not_exists(source, html_content)?;
    fs::copy(source, destination)?;
    Ok(())
}

fn get_menu_items(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut menu = Vec::new();
    let mut in_navi_area = false;

    for line in reader.lines() {
        let line = line?;
        if line.contains("<!--Navibereich-->") {
            in_navi_area = true;
        } else if in_navi_area {
            if line.starts_with("<!--#") {
                if let Some(item) = cut_string("<!--#", "-->", &line) {
                    menu.push(item);
                }
            } else {
                break;
            }
        }
    }

    Ok(menu)
}

fn cut_string(begin_str: &str, end_str: &str, input_str: &str) -> Option<String> {
    if let Some(begin_index) = input_str.find(begin_str) {
        if let Some(end_index) = input_str[begin_index + begin_str.len()..].find(end_str) {
            return Some(input_str[begin_index + begin_str.len()..begin_index + begin_str.len() + end_index].to_string());
        }
    }
    None
}

fn save(selected_item: &str, url_text: &str, name_text: &str) -> io::Result<()> {
    println!("The save function got the lines: {} and {}", url_text, name_text);
    let filepath = "Lesezeichen/Lesezeichen.html";
    let url_short = url_short(url_text);

    add_link_item(filepath, selected_item, url_text, name_text, &url_short)?;

    Ok(())
}

fn add_link_item(filepath: &str, selected_item: &str, url: &str, name: &str, url_short: &str) -> io::Result<()> {
    let position = format!("<!--{}-->", selected_item);

    let mut file = match OpenOptions::new().read(true).write(true).open(filepath) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e);
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", e);
        return Err(e);
    }

    let lines: Vec<&str> = contents.lines().collect();
    if let Some(index) = lines.iter().position(|&line| line.contains(&position)) {
        let new_line = format!("        <DT><A HREF=\"{}\" target=\"_blank\">{}</A>\n", url, name);
        let description = format!("        <DD>{}\n", url_short);
        let mut new_contents = lines[..index + 1].join("\n");
        new_contents.push('\n');
        new_contents.push_str(&new_line);
        new_contents.push_str(&description);
        new_contents.push_str(&lines[index + 1..].join("\n"));

        if let Err(e) = file.seek(SeekFrom::Start(0)) {
            eprintln!("Error seeking in file: {}", e);
            return Err(e);
        }
        if let Err(e) = file.set_len(0) {
            eprintln!("Error truncating file: {}", e);
            return Err(e);
        }
        if let Err(e) = file.write_all(new_contents.as_bytes()) {
            eprintln!("Error writing to file: {}", e);
            return Err(e);
        }
        // file.write_all(new_contents.as_bytes())
        //     .map_err(|e| { eprintln!("Error writing to file: {}", e); e })?;
        // // map_err keeps the code concise while still providing the same level of error handling, but it does not allow for early returns from the enclosing function.
    } else {
        eprintln!("Position not found for selected item: {}", selected_item);
        return Err(io::Error::new(io::ErrorKind::NotFound, "Position not found"));
    }

    Ok(())
}

fn url_short(url: &str) -> String {
    let start_pos = url.find("://").map_or(0, |pos| pos + "://".len());
    let end_pos = url[start_pos..].find('/').map_or(url.len(), |pos| start_pos + pos);

    // Extract the domain part of the URL
    let domain = &url[start_pos..end_pos];

    // Remove the 'www.' prefix if it exists
    let result = if domain.starts_with("www.") {
        &domain["www.".len()..]
    } else {
        domain
    };

    result.to_string()
}

fn add_menu_item(file_path: &str, item_name: &str) -> io::Result<()> {
    let navibereich = "<!--Navibereich-->";
    let textbereich = "<!--Textbereich-->";

    // Open the file for reading
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Find the positions to insert new content
    let navibereich_pos = contents.find(navibereich).ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Navibereich not found",
    ))? + navibereich.len();
    let textbereich_pos = contents.find(textbereich).ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Textbereich not found",
    ))? + textbereich.len();

    // Create the new content to be inserted
    let new_content_1 = format!("\n<!--#{}-->    <li><a href=\"#{}111\">{}</a></li>", item_name, item_name, item_name);
    let new_content_2 = format!(
        "\n    <p><a name=\"{}111\"></a><br />\n    <DT><H3>&nbsp;{}</H3>\n    <DL><p><!--{}-->\n    </DL>",
        item_name, item_name, item_name
    );

    // Insert the new content
    let new_contents = format!(
        "{}{}{}{}{}",
        &contents[..navibereich_pos],
        new_content_1,
        &contents[navibereich_pos..textbereich_pos],
        new_content_2,
        &contents[textbereich_pos..]
    );

    // Open the file for writing and truncate it
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    // Write the modified contents back to the file
    file.write_all(new_contents.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    // Make a copy of the original, just to be sure
    copy_file("Lesezeichen/Lesezeichen.html", "Lesezeichen/Lesezeichen_old_save.html")?;

    // Initialize GTK-4 application
    let app = Application::builder()
        .application_id("com.github.tornado3p9.lzedit")
        .build();

    app.connect_activate(|app| {
        // Create a new application window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Lesezeichen Editor")
            .default_width(560)
            .default_height(250)
            .resizable(false) // Make the window not resizable
            .build();

        // Create a vertical box (main box) to hold the top input and the split box
        let vbox = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .margin_start(6)
            .margin_end(6)
            .margin_top(6)
            .margin_bottom(6)
            .build();

        // Create the top text input element
        let top_input = Entry::builder()
            .placeholder_text("Type some url here...")
            .hexpand(true) // Make the input stretch from left to right
            .build();

        // Create a horizontal box to hold the listbox and the right-side elements
        let hbox = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .build();

        // Create the listbox for the left side https://gtk-rs.org/gtk4-rs/stable/latest/book/list_widgets.html
        let listbox = ListBox::builder()
            .hexpand(true) // Make the listbox expand horizontally
            .build();
        // for number in 0..=100 {
        //     let label = gtk4::Label::new(Some(&number.to_string()));
        //     listbox.append(&label);
        // }
        let scrolled_window = gtk4::ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
            .min_content_width(240) // Set minimum content width to 240
            .width_request(240) // Set fixed width to 240
            .vexpand(true) // Make the scrolled window expand vertically
            .child(&listbox)
            .build();

        // Create a vertical box for the right side
        let right_vbox = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .build();

        // Create the text input element for the right side
        let right_input = Entry::builder()
            .placeholder_text("Give a name to your url...")
            .hexpand(true) // Make the input stretch from left to right
            .build();

        // Create the two buttons for the bottom of the right side
        let button1 = Button::builder()
            .label("Save")
            .width_request(140)
            .height_request(70)
            .build();
        let button_spacer = Box::builder()
            .hexpand(true)
            .build();
        let button2 = Button::builder()
            .label("Exit")
            .width_request(140)
            .height_request(70)
            .build();

        // Add the buttons to a horizontal box
        let button_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(6)
            .build();
        button_box.append(&button1);
        button_box.append(&button_spacer);
        button_box.append(&button2);

        // Create an empty box with vertical expansion
        let spacer = Box::builder()
            .orientation(Orientation::Horizontal)
            .vexpand(true) // This will push the buttons to the bottom
            .build();

        // Pack everything into the right vertical box
        right_vbox.append(&right_input);
        right_vbox.append(&spacer);
        right_vbox.append(&button_box);

        // Pack the listbox and right vertical box into the horizontal box
        // hbox.append(&listbox);
        hbox.append(&scrolled_window);
        hbox.append(&right_vbox);

        // Pack the top input and horizontal box into the vertical box
        vbox.append(&top_input);
        vbox.append(&hbox);

        // Add the vertical box to the window
        window.set_child(Some(&vbox));

        // Setup actions
        /*************************/

        // Populate the ListBox with menu items
        while let Some(child) = listbox.first_child() {
            listbox.remove(&child);
        }
        if let Ok(menu_items) = get_menu_items("Lesezeichen/Lesezeichen.html") {
            for item in menu_items {
                let label = gtk4::Label::new(Some(&item));  // the Label is what we target for getting the text from the active list item
                let row = gtk4::ListBoxRow::new();
                row.set_child(Some(&label));
                listbox.append(&row);
            }
        }

        // Handle the buttons
        let window_clone = window.clone();

        let url_input = top_input.clone();
        let name_input = right_input.clone();
        // button1.connect_clicked(move |_| {
        //     if let Some(row) = listbox.selected_row() {
        //         if let Some(label) = row.child().and_then(|child| child.downcast::<gtk4::Label>().ok()) {
        //             let label_text = label.text().to_string();
        //             let url_text = url_input.text().to_string();
        //             let name_text = name_input.text().to_string();
        //             if !label_text.is_empty() && !url_text.is_empty() && !name_text.is_empty() {
        //                 // save(&label_text, &url_text, &name_text).expect("Failed to save the bookmark"); //not for production code but shorter
        //                 match save(&label_text, &url_text, &name_text) {
        //                     Ok(_) => println!("Bookmark saved successfully."),
        //                     Err(e) => eprintln!("Failed to save bookmark: {}", e),
        //                 }
        //                 url_input.set_text("");  // Clear the entry
        //                 name_input.set_text("");  // Clear the entry
        //             } else {
        //                 println!("url or name field empty!")
        //             }
        //         } else {
        //             println!("reading gtk4::Label to a variable did not work!")
        //         }
        //     } else {
        //         println!("You have not selected any listbox item!")
        //     }
        // });
        button1.connect_clicked(move |_| {
            let name_text = name_input.text().to_string();
            let url_text = url_input.text().to_string();
            if url_text.is_empty() && !name_text.is_empty() {
                match add_menu_item("Lesezeichen/Lesezeichen.html", &name_text) {
                    Ok(_) => println!("New menu item added successfully."),
                    Err(e) => eprintln!("Failed to add a new menu item: {}", e),
                }
                name_input.set_text("");  // Clear the entry
                let label = gtk4::Label::new(Some(&name_text));  // Create a label and prepend it to the list box
                listbox.prepend(&label);
            } else if let Some(row) = listbox.selected_row() {
                if let Some(label) = row.child().and_then(|child| child.downcast::<gtk4::Label>().ok()) {
                    let label_text = label.text().to_string();
                    if !label_text.is_empty() && !url_text.is_empty() && !name_text.is_empty() {
                        match save(&label_text, &url_text, &name_text) {
                            Ok(_) => println!("Bookmark saved successfully."),
                            Err(e) => eprintln!("Failed to save bookmark: {}", e),
                        }
                        url_input.set_text("");  // Clear the entry
                        name_input.set_text("");  // Clear the entry
                    } else {
                        println!("url or name field empty!")
                    }
                } else {
                    println!("reading gtk4::Label to a variable did not work!")
                }
            } else {
                println!("You have not selected any listbox item!")
            }
        });

        button2.connect_clicked(move |_| {
            window_clone.close();
        });

        /*************************/
        // Setup actions

        // Present the window
        window.present();
    });

    app.run();

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_short() {
        let url1 = "https://www.youtube.com/watch?v=6Fk1PgTxNZY";
        let url2 = "http://youtu.be/6Fk1PgTxNZY";
        let url3 = "www.youtube.com/watch?v=6Fk1PgTxNZY";
        let url4 = "youtube.com/watch?v=6Fk1PgTxNZY";
        let url5 = "www.youtube.com";
        let url6 = "youtube.com";
        let url7 = "ipfs://QmSomeHash";
        let url8 = "QmSomeHash";
        let url9 = "/QmSomeHash";
        let url10 = "https://github.com/Tornado3P9/otp/tree/dev/src";

        assert_eq!("youtube.com", url_short(url1));
        assert_eq!("youtu.be", url_short(url2));
        assert_eq!("youtube.com", url_short(url3));
        assert_eq!("youtube.com", url_short(url4));
        assert_eq!("youtube.com", url_short(url5));
        assert_eq!("youtube.com", url_short(url6));
        assert_eq!("QmSomeHash", url_short(url7));
        assert_eq!("QmSomeHash", url_short(url8));
        assert_eq!("", url_short(url9));
        assert_eq!("github.com", url_short(url10));
    }

}
