use std::process::exit;

use tigris_core::features::{api::FormResultsRequest, utils::send_notification};

use crate::db::{Clip, ClipType, Database};

pub fn handle_forms(request: FormResultsRequest) {
    let mut db = Database::get_db();

    match request.form_id.as_str() {
        "add-text-clip" => {
            let keyword = request.get_string_value("keyword").unwrap();
            let name = request.get_string_value("name").unwrap();
            let clip_content = request.get_string_value("clip").unwrap();

            db.add_clip(Clip::new(&keyword, &name, &clip_content, ClipType::Text));
            db.save();

            send_notification("Success", "Clip added successfully");
            exit(0);
        }
        "add-text-area-clip" => {
            let keyword = request.get_string_value("keyword").unwrap();
            let name = request.get_string_value("name").unwrap();
            let clip_content = request.get_string_value("clip").unwrap();

            db.add_clip(Clip::new(
                &keyword,
                &name,
                &clip_content,
                ClipType::TextArea,
            ));

            db.save();

            send_notification("Success", "Clip added successfully");
            exit(0);
        }
        "add-image-clip" => {
            let keyword = request.get_string_value("keyword").unwrap();
            let name = request.get_string_value("name").unwrap();
            let image_path = request.get_string_value("image-path").unwrap();

            db.add_clip(Clip::new(&keyword, &name, &image_path, ClipType::Image));
            db.save();

            send_notification("Success", "Clip added successfully");
            exit(0);
        }
        "delete-clip" => {
            let id = request.get_usize_value("clip").unwrap();

            db.clips = db
                .clips
                .iter()
                .filter(|clip| clip.id != id)
                .map(|clip| clip.to_owned())
                .collect();

            db.save();

            send_notification("Success", "Clip deleted successfully");
            exit(0);
        }
        "edit-clip" => {
            let args = &request.args;

            let id = args.first().unwrap().parse::<usize>().unwrap();

            let clip_type = args.get(1).unwrap();

            let keyword = request.get_string_value("keyword").unwrap();

            let name = request.get_string_value("name").unwrap();

            let content = request
                .get_string_value(if clip_type == "Image" {
                    "image-path"
                } else {
                    "clip"
                })
                .unwrap();

            db.clips = db
                .clips
                .iter()
                .map(|clip| {
                    let clip = clip.to_owned();

                    if clip.id == id {
                        Clip {
                            id: clip.id,
                            keyword: keyword.to_owned(),
                            name: name.to_owned(),
                            content: content.to_owned(),
                            clip_type: clip.clip_type,
                        }
                    } else {
                        clip
                    }
                })
                .collect();

            db.save();

            send_notification("Success", "Clip edited successfully");
            exit(0);
        }
        _ => {}
    }
}
