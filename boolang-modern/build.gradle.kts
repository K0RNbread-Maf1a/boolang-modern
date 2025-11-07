plugins {
    kotlin("jvm") version "1.9.22" apply false
    kotlin("multiplatform") version "1.9.22" apply false
    id("com.android.library") version "8.2.2" apply false
    id("org.jetbrains.dokka") version "1.9.10"
}

allprojects {
    group = "org.boolang"
    version = "0.1.0"

    repositories {
        mavenCentral()
        google()
    }
}

subprojects {
    apply(plugin = "org.jetbrains.dokka")
}

tasks.register("clean", Delete::class) {
    delete(rootProject.buildDir)
}

// Custom task to build Rust core before Kotlin
tasks.register("buildRustCore", Exec::class) {
    workingDir = file(".")
    
    if (System.getProperty("os.name").toLowerCase().contains("win")) {
        commandLine("cmd", "/c", "cargo", "build", "--release")
    } else {
        commandLine("cargo", "build", "--release")
    }
}

// Make Kotlin builds depend on Rust
subprojects {
    tasks.matching { it.name == "compileKotlin" }.configureEach {
        dependsOn(":buildRustCore")
    }
}
