12.5

mon_repertoire = directory : {
    path:"./exemples"
}



mon_header = header : {
    name:"nom",
    age:"age",
}

my_type = csv_type : {
    field_separator:[",",";"],
    line_separator:[nline,rline],
    text_separator:'"',
    with_header:true,
}

mon_fichier = file : {
    name:"fichier1.csv",
    directory:repertoire1,
    format:ascii,
    type:my_type
}

ma_data_source = file_data_source {
    file:mon_fichier,
}

data_set = ma_data_source.read()

