<?xml version="1.0" encoding="utf-8"?>
<xsl:transform version="1.0"
    xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
    xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
    xmlns:schema="http://schema.org/">
    <!-- xsltproc -o static/sitemap.xml scripts/rdf-to-sitemap.xsl blog.rdf -->

    <xsl:template match="/rdf:RDF">
        <urlset>
            <url>
                <loc>https://blog.adrianistan.eu</loc>
            </url>
            <xsl:for-each select="schema:BlogPost">
                <url>
                    <loc><xsl:value-of select="@rdf:about"/></loc>
                </url>
            </xsl:for-each>
        </urlset>
    </xsl:template>
</xsl:transform>