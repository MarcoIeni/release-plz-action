FROM marcoieni/release-plz:sha-106dda5

LABEL repository="https://github.com/MarcoIeni/release-plz-action"
LABEL homepage="https://github.com/MarcoIeni/release-plz"

COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
