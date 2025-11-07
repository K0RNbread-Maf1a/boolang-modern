plugins {
    kotlin("jvm")
    `java-library`
}

dependencies {
    // ASM for bytecode generation
    implementation("org.ow2.asm:asm:9.6")
    implementation("org.ow2.asm:asm-tree:9.6")
    implementation("org.ow2.asm:asm-util:9.6")
    
    // Kotlin compiler APIs
    implementation("org.jetbrains.kotlin:kotlin-compiler-embeddable:1.9.22")
    
    // JNA for Rust FFI
    implementation("net.java.dev.jna:jna:5.14.0")
    
    // CLI
    implementation("com.github.ajalt.clikt:clikt:4.2.1")
    
    // Logging
    implementation("io.github.oshai:kotlin-logging-jvm:6.0.1")
    implementation("ch.qos.logback:logback-classic:1.4.14")
    
    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.1")
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(17)
}

// Copy Rust shared library after build
tasks.register<Copy>("copyRustLib") {
    dependsOn(":buildRustCore")
    
    val rustTarget = if (System.getProperty("os.name").toLowerCase().contains("win")) {
        "target/release/boolang_modern.dll"
    } else if (System.getProperty("os.name").toLowerCase().contains("mac")) {
        "target/release/libboolang_modern.dylib"
    } else {
        "target/release/libboolang_modern.so"
    }
    
    from(rootProject.file(rustTarget))
    into(buildDir.resolve("libs"))
}

tasks.named("build") {
    dependsOn("copyRustLib")
}
