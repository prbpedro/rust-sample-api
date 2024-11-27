
FROM amazoncorretto:23-alpine-jdk as build
WORKDIR /app
COPY gradlew ./gradlew
COPY gradle ./gradle
COPY build.gradle ./build.gradle
COPY settings.gradle ./settings.gradle
COPY src ./src
RUN chmod +x ./gradlew
RUN ./gradlew clean build --no-daemon

FROM amazoncorretto:23-alpine
WORKDIR /app
COPY --from=build /app/build/libs/bff-0.0.1-SNAPSHOT.jar app.jar
EXPOSE 8080
ENTRYPOINT ["java", "-jar", "app.jar"]