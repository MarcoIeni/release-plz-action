FROM marcoieni/release-plz:sha-7d1000c

LABEL repository="https://github.com/MarcoIeni/release-plz-action"
LABEL homepage="https://github.com/MarcoIeni/release-plz"

COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
