#include <napi.h>

struct TSLanguage;
extern "C" TSLanguage *tree_sitter_csv();
extern "C" TSLanguage *tree_sitter_psv();
extern "C" TSLanguage *tree_sitter_tsv();

static const napi_type_tag LANGUAGE_TYPE_TAG = {
    0x8AF2E5212AD58ABF, 0xD5006CAD83ABBA16
};

static Napi::Object Language(Napi::Env env, const char *name, TSLanguage *pointer) {
    auto result = Napi::Object::New(env);
    auto language = Napi::External<TSLanguage>::New(env, pointer);
    language.TypeTag(&LANGUAGE_TYPE_TAG);
    result["name"] = Napi::String::New(env, name);
    result["language"] = language;
    return result;
}

static Napi::Object Init(Napi::Env env, Napi::Object exports) {
    exports["csv"] = Language(env, "csv", tree_sitter_csv());
    exports["psv"] = Language(env, "psv", tree_sitter_psv());
    exports["tsv"] = Language(env, "tsv", tree_sitter_tsv());
    return exports;
}

NODE_API_MODULE(tree_sitter_csv_binding, Init)
