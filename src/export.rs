// Cada día, si se ha actualizado la base de datos, modificar el fichero RDF, regenerar RSS y Sitemap
// Debería ser un proceso independiente la generación del RDF, hecho en Python
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::{comment, post, tag};

use std::fs::*;
use std::io::Write;

use chrono::prelude::*;

// Thread aparte
// Obtener configuración de Rocket.toml

pub fn export(database_url: &str) {
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    // GENERICO RDF
    let mut rdf = String::new();
    rdf.push_str(
        r#"<?xml version="1.0" encoding="utf-8" ?>
    <rdf:RDF
    xmlns:extra="http://adrianistan.eu/" 
    xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
    xmlns:schema="http://schema.org/">

    <schema:Blog rdf:about="https://blog.adrianistan.eu">
        <schema:url rdf:datatype="http://schema.org/URL">https://blog.adrianistan.eu</schema:url>
        <schema:name>Adrianistán</schema:name>
        <schema:description>El blog de Adrián Arroyo</schema:description>
        <schema:license>http://creativecommons.org/licenses/by/4.0/</schema:license>
        <schema:inLanguage>es</schema:inLanguage>
        <schema:author rdf:resource='#AdrianArroyo'/>
        <schema:copyrightYear>2019</schema:copyrightYear>
        <schema:copyrightHolder rdf:resource='#AdrianArroyo'/>"#,
    );
    // LISTADO POSTS
    let post_urls = post::table
        .select(post::slug)
        .filter(post::status.eq("published"))
        .load::<String>(&conn)
        .unwrap();
    for url in post_urls {
        rdf.push_str(&format!(
            "\n<schema:blogPost rdf:resource='https://blog.adrianistan.eu/{}'/>",
            url
        ));
    }
    rdf.push_str(
        r#"
    </schema:Blog>
    <schema:Person rdf:about='#AdrianArroyo'>
        <schema:email>adrian.arroyocalle@gmail.com</schema:email>
        <schema:name>Adrián Arroyo Calle</schema:name>
    </schema:Person>
    "#,
    );

    // POSTS (con IDs de COMENTARIOS)
    let posts = post::table
        .filter(post::status.eq("published"))
        .load::<crate::models::Post>(&conn)
        .unwrap();
    for post in posts {
        let date: DateTime<FixedOffset> =
            DateTime::from_utc(post.date, FixedOffset::west_opt(0).unwrap());
        rdf.push_str(&format!(
            r#"
        <schema:BlogPost rdf:about='https://blog.adrianistan.eu/{}'>
            <schema:name>{}</schema:name>
            <schema:articleBody><![CDATA[{}]]></schema:articleBody>
            <schema:author rdf:resource='#AdrianArroyo'/>
            <schema:dateCreated rdf:datatype="http://schema.org/DateTime">{}</schema:dateCreated>
            <extra:dateRFC822>{}</extra:dateRFC822>
        "#,
            post.slug,
            post.title,
            post.content,
            post.date,
            date.to_rfc2822()
        ));
        let comment_ids = comment::table
            .select(comment::id)
            .filter(
                comment::post_id
                    .eq(post.id)
                    .and(comment::status.eq("approved")),
            )
            .load::<i32>(&conn)
            .unwrap();
        for comment_id in comment_ids {
            rdf.push_str(&format!(
                "\n<schema:comment rdf:resource='#comment{}' />",
                comment_id
            ));
        }
        let tags = tag::table
            .select(tag::name)
            .filter(tag::post_id.eq(post.id))
            .load::<String>(&conn)
            .unwrap();
        for tag in tags {
            rdf.push_str(&format!("\n<schema:keywords>{}</schema:keywords>", tag))
        }
        rdf.push_str("\n</schema:BlogPost>");
    }

    // COMENTARIOS
    let comments = comment::table
        .filter(comment::status.eq("approved"))
        .load::<crate::models::Comment>(&conn)
        .unwrap();
    for comment in comments {
        rdf.push_str(&format!(
            r#"
        <schema:Comment rdf:about='#comment{}'>
            <schema:articleBody><![CDATA[{}]]></schema:articleBody>
            <schema:dateCreated rdf:datatype="http://schema.org/DateTime">{}</schema:dateCreated>
            <schema:author>
                <schema:Person>
                    <schema:name>{}</schema:name>"#,
            comment.id, comment.content, comment.date, comment.author_name
        ));
        if let Some(url) = comment.author_url {
            if url != "" {
                rdf.push_str(&format!("\n<schema:url>{}</schema:url>", url));
            }
        }
        rdf.push_str(
            r#"
                </schema:Person>
            </schema:author>
        </schema:Comment>
        "#,
        );
    }

    rdf.push_str("</rdf:RDF>");

    // SAVE
    let mut file = File::create("blog.rdf").unwrap();
    file.write_all(rdf.as_bytes()).unwrap();

    // LLAMAR SCRIPTS
    /*let _rss = Command::new("xsltproc")
        .arg("-o")
        .arg("static/rss.xml")
        .arg("scripts/rdf-to-rss.xsl")
        .arg("blog.rdf")
        .spawn()
        .expect("Failed to create RSS");

    let _sitemap = Command::new("xsltproc")
        .arg("-o")
        .arg("static/sitemap.xml")
        .arg("scripts/rdf-to-sitemap.xsl")
        .arg("blog.rdf")
        .spawn()
        .expect("Failed to create Sitemap");

    let _programacion = Command::new("xsltproc")
        .arg("-o")
        .arg("static/programacion.rss.xml")
        .arg("scripts/rdf-to-programacion-rss.xsl")
        .arg("blog.rdf")
        .spawn()
        .expect("Failed to create Programacion RSS");*/
}
