use nexus::{
    expr::{
        get::get,
        ops::{Filter, NameField, Table},
    },
    poll::source::yougov::YouGov,
};

#[test]
pub fn test_get_basic_expression() {
    get().names();
}

#[test]
pub fn test_get_names_expression() {
    let expr = get().names().where_as(NameField::FirstName, "John");

    assert_eq!(expr.table(), Some(Table::People));
    assert_eq!(
        expr.filters(),
        &[Filter::Name {
            field: NameField::FirstName,
            value: "John".to_string()
        }]
    );
}

#[test]
pub fn test_get_polls_expression() {
    let expr = get()
        .polls()
        .from_soure(YouGov)
        .from("04-15-2025")
        .to("04-15-2026");

    assert_eq!(expr.table(), Some(Table::Polls));
    assert_eq!(
        expr.filters(),
        &[
            Filter::PollSource {
                source_name: "YouGov"
            },
            Filter::PollFrom {
                date: "04-15-2025".to_string()
            },
            Filter::PollTo {
                date: "04-15-2026".to_string()
            },
        ]
    );
}
