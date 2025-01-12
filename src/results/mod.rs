use std::{path::Path, process::exit};

use sniffer_rs::sniffer::Sniffer;
use tigris_rs::features::{
    actions::{
        CopyImageAction, CopyTextAction, Field, FieldValidation, FileSystemField, OpenFormAction,
        ResultAction, SelectField, SelectFieldValue, TextAreaField, TextField,
    },
    api::{send_search_results, GetResultsRequest},
    search::get_search_query,
    search_results::SearchResult,
};

use crate::{
    db::{ClipType, Database},
    icons::get_icon_path,
    EXTENSION_ID,
};

pub fn handle_results(request: GetResultsRequest) {
    let search_query = get_search_query(&request.search_text);
    let keyword = search_query.keyword;
    let search_text = if keyword.is_some() {
        search_query.search_text
    } else {
        request.search_text.clone()
    };
    let mut results = Vec::<SearchResult>::new();
    let sniffer = Sniffer::new();
    let db = Database::get_db();

    if search_text.is_empty() {
        let add_text_clip_result = SearchResult::new("Add Text Clip")
            .set_description("Add a text clip")
            .set_icon_color("accent")
            .set_icon_path(&get_icon_path("plus"))
            .set_action(&ResultAction::new_open_form_action(
                &OpenFormAction::new(EXTENSION_ID, "add-text-clip", "Add Text Clip", "Add Clip")
                    .add_field(&Field::new_text_field(
                        "keyword",
                        "Keyword",
                        "The keyword of the clip",
                        &TextField::new("")
                            .set_validation(&FieldValidation::new().set_not_empty(true)),
                    ))
                    .add_field(&Field::new_text_field(
                        "name",
                        "Name",
                        "The name of the clip",
                        &TextField::new("")
                            .set_validation(&FieldValidation::new().set_not_empty(true)),
                    ))
                    .add_field(&Field::new_text_field(
                        "clip",
                        "Clip",
                        "The text of the clip",
                        &TextField::new("")
                            .set_validation(&FieldValidation::new().set_not_empty(true)),
                    )),
            ));

        let add_text_area_clip_result = SearchResult::new("Add Text Area Clip")
            .set_description("Add a text area clip")
            .set_icon_color("accent")
            .set_icon_path(&get_icon_path("plus"))
            .set_action(&ResultAction::new_open_form_action(
                &OpenFormAction::new(
                    EXTENSION_ID,
                    "add-text-area-clip",
                    "Add Text Area Clip",
                    "Add Clip",
                )
                .add_field(&Field::new_text_field(
                    "keyword",
                    "Keyword",
                    "The keyword of the clip",
                    &TextField::new("").set_validation(&FieldValidation::new().set_not_empty(true)),
                ))
                .add_field(&Field::new_text_field(
                    "name",
                    "Name",
                    "The name of the clip",
                    &TextField::new("").set_validation(&FieldValidation::new().set_not_empty(true)),
                ))
                .add_field(&Field::new_text_area_field(
                    "clip",
                    "Clip",
                    "The text of the clip",
                    &TextAreaField::new("")
                        .set_validation(&FieldValidation::new().set_not_empty(true)),
                )),
            ));

        let add_image_clip_result = SearchResult::new("Add Image Clip")
            .set_description("Add a image clip")
            .set_icon_color("accent")
            .set_icon_path(&get_icon_path("plus"))
            .set_action(&ResultAction::new_open_form_action(
                &OpenFormAction::new(EXTENSION_ID, "add-image-clip", "Add Image Clip", "Add Clip")
                    .add_field(&Field::new_text_field(
                        "keyword",
                        "Keyword",
                        "The keyword of the clip",
                        &TextField::new("")
                            .set_validation(&FieldValidation::new().set_not_empty(true)),
                    ))
                    .add_field(&Field::new_text_field(
                        "name",
                        "Name",
                        "The name of the clip",
                        &TextField::new(""),
                    ))
                    .add_field(&Field::new_file_system_field(
                        "image-path",
                        "Image",
                        "Select the image path",
                        &FileSystemField::new(Path::new(""))
                            .set_not_empty(true)
                            .add_filter("png")
                            .add_filter("webp")
                            .add_filter("jpg")
                            .add_filter("jpeg"),
                    )),
            ));

        results.push(add_text_clip_result);
        results.push(add_text_area_clip_result);
        results.push(add_image_clip_result);

        if !db.clips.is_empty() {
            let delete_clip_result = SearchResult::new("Delete Clip")
                .set_description("Delete a clip")
                .set_icon_path(&get_icon_path("trash"))
                .set_icon_color("accent")
                .set_action(&ResultAction::new_open_form_action(
                    &OpenFormAction::new(EXTENSION_ID, "delete-clip", "Delete Clip", "Delete")
                        .add_field(&Field::new_select_field(
                            "clip",
                            "Clip",
                            "Select the clip you wish to delete",
                            &SelectField::new(
                                &db.clips.first().unwrap().id.to_string(),
                                &db.clips
                                    .iter()
                                    .map(|clip| {
                                        SelectFieldValue::new(&clip.id.to_string(), &clip.name)
                                    })
                                    .collect(),
                            ),
                        )),
                ));

            results.push(delete_clip_result);
        }

        send_search_results(&results);
        exit(0);
    }

    if let Some(keyword) = &keyword {
        if keyword == "e" || keyword == "edit" {
            let mut edit_results = db
                .clips
                .iter()
                .filter(|clip| {
                    sniffer.matches(&clip.name, &search_text)
                        || sniffer.matches(&clip.keyword, &search_text)
                })
                .map(|clip| {
                    SearchResult::new(&format!("Edit {}", &clip.name))
                        .set_description("Edit the clip content")
                        .set_icon_path(&get_icon_path("pencil"))
                        .set_icon_color("accent")
                        .set_action(&ResultAction::new_open_form_action(
                            &OpenFormAction::new(EXTENSION_ID, "edit-clip", "Edit Clip", "Save")
                                .add_arg(&clip.id.to_string())
                                .add_arg(&clip.clip_type.to_string())
                                .add_field(&Field::new_text_field(
                                    "keyword",
                                    "Keyword",
                                    "The clip keyword",
                                    &TextField::new(&clip.keyword),
                                ))
                                .add_field(&Field::new_text_field(
                                    "name",
                                    "Name",
                                    "The clip name",
                                    &TextField::new(&clip.name),
                                ))
                                .add_field(&if clip.clip_type == ClipType::Image {
                                    Field::new_file_system_field(
                                        "image-path",
                                        "Image",
                                        "Select the image path",
                                        &FileSystemField::new(Path::new(&clip.content))
                                            .set_not_empty(true)
                                            .add_filter("png")
                                            .add_filter("webp")
                                            .add_filter("jpg")
                                            .add_filter("jpeg"),
                                    )
                                } else {
                                    Field::new_text_area_field(
                                        "clip",
                                        "Clip",
                                        "The text of the clip",
                                        &TextAreaField::new(&clip.content).set_validation(
                                            &FieldValidation::new().set_not_empty(true),
                                        ),
                                    )
                                }),
                        ))
                })
                .collect::<Vec<SearchResult>>();

            results.append(&mut edit_results);
        }
    };

    let mut clips = db
        .clips
        .iter()
        .filter(|clip| {
            sniffer.matches(&clip.name, &search_text)
                || if let Some(keyword) = &keyword {
                    &clip.keyword == keyword
                } else {
                    &clip.keyword == &search_text
                }
        })
        .map(|clip| {
            let splitted_search = search_text.split_whitespace();
            let mut parsed_content = clip.content.to_owned();

            for (index, word) in splitted_search.enumerate() {
                let pattern = format!("{{%{}}}", index);
                parsed_content = parsed_content.replace(&pattern, &word);
            }

            parsed_content = parsed_content.replace("{%s}", &search_text);

            match clip.clip_type {
                crate::db::ClipType::Text => {
                    SearchResult::new(&format!("({}) {}", &clip.keyword, &clip.name))
                        .set_description(&parsed_content)
                        .set_icon_path(&get_icon_path("copy"))
                        .set_icon_color("accent")
                        .set_action(&ResultAction::new_copy_text_action(&CopyTextAction::new(
                            &parsed_content,
                        )))
                }
                crate::db::ClipType::TextArea => {
                    SearchResult::new(&format!("({}) {}", &clip.keyword, &clip.name))
                        .set_description(&parsed_content)
                        .set_icon_path(&get_icon_path("copy"))
                        .set_icon_color("accent")
                        .set_action(&ResultAction::new_copy_text_action(&CopyTextAction::new(
                            &parsed_content,
                        )))
                }
                crate::db::ClipType::Image => {
                    SearchResult::new(&format!("({}) {}", &clip.keyword, &clip.name))
                        .set_description("Copy image")
                        .set_icon_path(Path::new(&clip.content))
                        .set_action(&ResultAction::new_copy_image_action(&CopyImageAction::new(
                            Path::new(&clip.content),
                        )))
                }
            }
        })
        .collect();

    results.append(&mut clips);

    send_search_results(&results);

    exit(0)
}
