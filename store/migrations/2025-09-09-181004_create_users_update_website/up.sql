-- Your SQL goes here
ALTER TABLE "website" ADD COLUMN "userId" TEXT NOT NULL;

ALTER TABLE "websiteTick" ADD COLUMN  "createdAt" TIMESTAMP(3) NOT NULL
DEFAULT CURRENT_TIMESTAMP;

CREATE TABLE "User" (
    "id" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "password" TEXT NOT NULL,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "website" ADD CONSTRAINT "website_user_id_fkey" FOREIGN KEY ("userId") 
REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;