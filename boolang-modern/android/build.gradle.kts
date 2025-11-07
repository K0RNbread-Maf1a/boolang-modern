plugins {
    id("com.android.library")
    kotlin("android")
}

android {
    namespace = "org.boolang.android"
    compileSdk = 34

    defaultConfig {
        minSdk = 24
        targetSdk = 34

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
        
        ndk {
            // Include Rust compiled native libraries
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86", "x86_64")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    
    kotlinOptions {
        jvmTarget = "17"
    }
    
    // Configure native library location
    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }
}

dependencies {
    implementation(project(":kotlin:runtime"))
    implementation(project(":kotlin:stdlib"))
    
    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
    
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
}

// Task to copy Rust native libraries for Android
tasks.register("copyRustNativeLibs") {
    dependsOn(":buildRustCore")
    
    doLast {
        val jniLibsDir = file("src/main/jniLibs")
        jniLibsDir.mkdirs()
        
        // Map Android ABIs to Rust targets
        val abiTargets = mapOf(
            "arm64-v8a" to "aarch64-linux-android",
            "armeabi-v7a" to "armv7-linux-androideabi",
            "x86" to "i686-linux-android",
            "x86_64" to "x86_64-linux-android"
        )
        
        abiTargets.forEach { (abi, rustTarget) ->
            val targetDir = rootProject.file("target/$rustTarget/release")
            val destDir = file("src/main/jniLibs/$abi")
            destDir.mkdirs()
            
            if (targetDir.exists()) {
                copy {
                    from(targetDir)
                    into(destDir)
                    include("*.so")
                }
            }
        }
    }
}

tasks.named("preBuild") {
    dependsOn("copyRustNativeLibs")
}
