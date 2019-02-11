<?xml version="1.0" encoding="utf-8" ?>
<xsl:transform version="1.0" 
    xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
    xmlns:schema="http://schema.org/">
    <!-- xsltproc -o static/programacion.rss.xml scripts/rdf-to-programacion-rss.xsl blog.rdf -->
<xsl:output cdata-section-elements="description"/>
<xsl:template match="/rdf:RDF">
    <rss version="2.0">
        <channel>
            <title><xsl:value-of select="schema:Blog/schema:name"/></title>
            <link><xsl:value-of select="schema:Blog/schema:url"/></link>
            <description><xsl:value-of select="schema:Blog/schema:description"/></description>
            <xsl:for-each select="schema:BlogPost">
                <xsl:sort select="schema:dateCreated" order="descending"/>
                <xsl:if test="schema:keywords/text() = 'programacion'">
                    <item>
                        <title><xsl:value-of select="schema:name"/></title>
                        <link><xsl:value-of select="@rdf:about"/></link>
                        <description><xsl:value-of select="schema:articleBody" /></description>
                    </item>
                </xsl:if>
            </xsl:for-each>
        </channel>
    </rss>
</xsl:template>

</xsl:transform>