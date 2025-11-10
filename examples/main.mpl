mon_repertoire = directory : {
    path:"./exemples"
}

my_type = csv_type : {
    field_separator:[",",";"],
    line_separator:[nline,rline],
    quote_character:"\"",
    escape_character:"\\""
}

mon_fichier = file : {
    name:"fichier1.csv",
    directory:repertoire1,
    type
}

ma_data_source = data_source {
        type:
        file_name = "data1.csv",
        directory = ".\exemple.csv"
load_csv(
file_name = "data.csv"
