-- Your SQL goes here
CREATE TYPE "websiteStatus" AS ENUM ('Up', 'Down', 'Unknown');

CREATE TABLE "website" (
    "id" TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "timeAdded" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "website_pkey" PRIMARY KEY ("id")
    );

CREATE TABLE "Region" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,

    CONSTRAINT "Region_pkey" PRIMARY KEY ("id")

    );

CREATE TABLE "websiteTick" (
    "id" TEXT NOT NULL,
    "response_time_ms" INTEGER NOT NULL,
    "status" "websiteStatus" NOT NULL,
    "region_id" TEXT NOT NULL,
    "website_id" TEXT NOT NULL,

    CONSTRAINT "websiteTick_pkey" PRIMARY KEY ("id")
);


ALTER TABLE "websiteTick" ADD CONSTRAINT "websiteTick_website_id_fkey" 
FOREIGN KEY ("website_id") REFERENCES "website"("id") ON DELETE RESTRICT ON
UPDATE CASCADE;

ALTER TABLE "websiteTick" ADD CONSTRAINT "websiteTick_region_id_fkey" 
FOREIGN KEY ("region_id") REFERENCES "Region"("id") ON DELETE RESTRICT ON
UPDATE CASCADE;
