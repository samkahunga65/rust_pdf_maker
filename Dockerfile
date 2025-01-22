FROM ubuntu

COPY ./target/release/pdf_maker .

# RUN mkdir ./opt/pdfmaker

# COPY pdf_maker ./opt/pdfmaker/

RUN chmod +x ./pdf_maker

EXPOSE 8080

# RUN chmod +x ./opt/pdfmaker/pdf_maker

CMD ["./pdf_maker"]
