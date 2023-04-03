-- Your SQL goes here
CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email_address VARCHAR NOT NULL
);

CREATE TABLE workouts 
(
    id SERIAL PRIMARY KEY,
    workout_name VARCHAR NOT NULL,
    workout_description TEXT,
    muscle_group TEXT[],
    split_group TEXT[]
);

CREATE TABLE users_workouts 
(
    user_id INT NOT NULL,
    workout_id INT NOT NULL,
    PRIMARY KEY (user_id, workout_id),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT fk_workout FOREIGN KEY (workout_id) REFERENCES workouts(id)
);

INSERT INTO users(full_name, username, password, email_address) VALUES ('Ng Kok Sheng', 'kawakaze0', 'qwertz999', 'ngkoksheng1999@gmail.com');
INSERT INTO users(full_name, username, password, email_address) VALUES ('Soo Yee Khee', 'kimiko99', 'qwertz999', 'sooyeekhee@gmail.com');