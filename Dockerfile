FROM marcoieni/release-plz:sha-de396cd

LABEL repository="https://github.com/MarcoIeni/release-plz-action"
LABEL homepage="https://github.com/MarcoIeni/release-plz"

COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
