FROM ubuntu

COPY ./release/ord_0.5.1_musl /usr/local/bin/ord

EXPOSE 7003
ENTRYPOINT ["ord"]
