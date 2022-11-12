create table _prisma_migrations
(
    id                  varchar(36)                            not null
        primary key,
    checksum            varchar(64)                            not null,
    finished_at         timestamp with time zone,
    migration_name      varchar(255)                           not null,
    logs                text,
    rolled_back_at      timestamp with time zone,
    started_at          timestamp with time zone default now() not null,
    applied_steps_count integer                  default 0     not null
);

alter table _prisma_migrations
    owner to postgres;

create table "Keywords"
(
    id         serial
        primary key,
    keyword    text                                   not null,
    created_at timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at timestamp(3) default CURRENT_TIMESTAMP not null
);

alter table "Keywords"
    owner to postgres;

create table site
(
    id          serial
        primary key,
    name        text                                   not null,
    url         text                                   not null,
    created_at  timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at  timestamp(3) default CURRENT_TIMESTAMP not null,
    last_online timestamp(3),
    last_scrape timestamp(3) default CURRENT_TIMESTAMP not null
);

alter table site
    owner to postgres;

create unique index site_url_key
    on site (url);

create table page
(
    id          serial
        primary key,
    url         text                                   not null,
    created_at  timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at  timestamp(3) default CURRENT_TIMESTAMP not null,
    last_scrape timestamp(3) default CURRENT_TIMESTAMP not null,
    site_id     integer                                not null
        references site
            on update cascade on delete restrict
);

alter table page
    owner to postgres;

create table link
(
    id           serial
        primary key,
    url          text                                   not null,
    created_at   timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at   timestamp(3) default CURRENT_TIMESTAMP not null,
    scraped      boolean      default false             not null,
    last_scraped timestamp(3),
    site_id      integer
                                                        references site
                                                            on update cascade on delete set null
);

alter table link
    owner to postgres;

create unique index link_url_key
    on link (url);

create table flag
(
    id      serial
        primary key,
    name    text not null,
    site_id integer
                 references site
                     on update cascade on delete set null
);

alter table flag
    owner to postgres;

create table search
(
    id         serial
        primary key,
    query      text                                   not null,
    created_at timestamp(3) default CURRENT_TIMESTAMP not null,
    updated_at timestamp(3) default CURRENT_TIMESTAMP not null
);

alter table search
    owner to postgres;

create table search_result
(
    id           serial
        primary key,
    position     integer                                not null,
    page_id      integer                                not null
        references page
            on update cascade on delete restrict,
    clicked      boolean      default false             not null,
    last_clicked timestamp(3) default CURRENT_TIMESTAMP not null,
    result_type  result_type                            not null,
    search_id    integer                                not null
        references search
            on update cascade on delete restrict
);

alter table search_result
    owner to postgres;

create table "_linkTopage"
(
    "A" integer not null
        references link
            on update cascade on delete cascade,
    "B" integer not null
        references page
            on update cascade on delete cascade
);

alter table "_linkTopage"
    owner to postgres;

create unique index "_linkTopage_AB_unique"
    on "_linkTopage" ("A", "B");

create index "_linkTopage_B_index"
    on "_linkTopage" ("B");

create table "_flagTopage"
(
    "A" integer not null
        references flag
            on update cascade on delete cascade,
    "B" integer not null
        references page
            on update cascade on delete cascade
);

alter table "_flagTopage"
    owner to postgres;

create unique index "_flagTopage_AB_unique"
    on "_flagTopage" ("A", "B");

create index "_flagTopage_B_index"
    on "_flagTopage" ("B");

create table "_flagTokeyword"
(
    "A" integer not null
        references flag
            on update cascade on delete cascade,
    "B" integer not null
        references "Keywords"
            on update cascade on delete cascade
);

alter table "_flagTokeyword"
    owner to postgres;

create unique index "_flagTokeyword_AB_unique"
    on "_flagTokeyword" ("A", "B");

create index "_flagTokeyword_B_index"
    on "_flagTokeyword" ("B");

