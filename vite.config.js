import {defineConfig} from "vite";
import EnvironmentPlugin from "vite-plugin-environment";
import path from "path";
import tsconfigPaths from 'vite-tsconfig-paths';

import dfxJson from "./dfx.json";

let localCanisters, prodCanisters, canisters;

let localEnv = true;
let network = "local";

function initCanisterIds() {
    try {
        localCanisters = require(path.resolve(
            ".dfx",
            "local",
            "canister_ids.json"
        ));
    } catch (error) {
        console.log("No local canister_ids.json found. Continuing production");
    }
    try {
        prodCanisters = require(path.resolve("canister_ids.json"));
        localEnv = false;
    } catch (error) {
        console.log("No production canister_ids.json found. Continuing with local");
    }

    network = process.env.NODE_ENV === "production" && !localEnv ? "ic" : "local";

    canisters = network === "local" || localEnv ? localCanisters : prodCanisters;
    for (const canister in canisters) {
        process.env[canister.toUpperCase() + "_CANISTER_ID"] =
            canisters[canister][network];
    }
}

const isDevelopment = process.env.NODE_ENV !== "production" || localEnv;

initCanisterIds();

const asset_entry = path.join("src", "frontend", "assets", "frontend", "index.html");

// List of all aliases for canisters
// This will allow us to: import { canisterName } from "canisters/canisterName"
const aliases = Object.entries(dfxJson.canisters).reduce(
    (acc, [name, _value]) => {
        // Get the network name, or `local` by default.
        const networkName = process.env["DFX_NETWORK"] || "local";
        const outputRoot = path.join(
            __dirname,
            ".dfx",
            networkName,
            "canisters",
            name
        );

        return {
            ...acc,
            ["canisters/" + name]: path.join(outputRoot, "index" + ".js"),
        };
    },
    {}
);

// Generate canister ids, required by the generated canister code in .dfx/local/canisters/*
// This strange way of JSON.stringifying the value is required by vite
const canisterDefinitions = Object.entries(canisters).reduce(
    (acc, [key, val]) => ({
        ...acc,
        [`process.env.${key.toUpperCase()}_CANISTER_ID`]: isDevelopment
            ? JSON.stringify(val.local)
            : JSON.stringify(val.ic),
    }),
    {}
);

export default defineConfig({

    test: {
        globals: true,
        environment: "jsdom",
        css: true,
        setupFiles:"./tests_setup.js",

        transform: {
            "^.+\\.js$": "babel-jest",
            "^.+\\.ts$": "ts-jest",
        },
        moduleFileExtensions: ["js", "ts", "json"],
        moduleNameMapper: {
            "^canisters/(.*)": "./.dfx/local/canisters/$1",
        }
    },
    build: {
        outDir: "build",
    },
    resolve: {
        alias: {
            // Here we tell Vite the "fake" modules that we want to define
            ...aliases,
        },
    },
    server: {
        fs: {
            allow: ["."],
        },
        proxy: {
            // This proxies all http requests made to /api to our running dfx instance
            "/api": {
                target: "http://localhost:8080",
                changeOrigin: true,
                rewrite: (path) => path.replace(/^\/api/, "/api"),
            },
        },
    },
    define: {
        // Here we can define global constants
        // This is required for now because the code generated by dfx relies on process.env being set
        global: "window",
        ...canisterDefinitions,
        "process.env.NODE_ENV": JSON.stringify(
            isDevelopment ? "development" : "production"
        ),
    },
    plugins: [
        tsconfigPaths(),
        EnvironmentPlugin("all", {prefix: "CANISTER_"}),
        EnvironmentPlugin("all", {prefix: "DFX_"}),
        EnvironmentPlugin({BACKEND_CANISTER_ID: ""}),
    ],
});
