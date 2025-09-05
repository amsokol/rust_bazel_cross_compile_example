package com.example.rustlib;

import java.util.Locale;

public class Arch {
    public static String normalizedArch() {
        String arch = System.getProperty("os.arch");
        if (arch == null) return "unknown";
        arch = arch.toLowerCase(Locale.ROOT);

        // Map common aliases
        if (arch.equals("aarch64") || arch.equals("arm64")) {
            return "arm64";
        }
        if (arch.equals("amd64") || arch.equals("x86_64") || arch.equals("x64")) {
            return "amd64";
        }

        // Not one of the two requested
        return "unknown";
    }
}