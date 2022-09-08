INPUT_ADOC=rewards-design.adoc
asciidoctor -r asciidoctor-diagram -r asciidoctor-mathematical --backend html --out-file - $INPUT_ADOC | \
# asciidoctor -r asciidoctor-diagram --backend html --out-file - $INPUT_ADOC | \
pandoc --from html --to markdown_strict --output $INPUT_ADOC.md