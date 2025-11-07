rootProject.name = "boolang-modern"

include(":kotlin:compiler")
include(":kotlin:runtime")
include(":kotlin:stdlib")
include(":android")
include(":gradle-plugins")

pluginManagement {
    repositories {
        gradlePluginPortal()
        google()
        mavenCentral()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}
