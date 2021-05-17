#include <stdio.h>
#include <inttypes.h>

#include <ftml.h>

static const char *meta_type(ftml_html_meta_type type)
{
	switch (type) {
	case META_NAME:
		return "Name";
	case META_HTTP_EQUIV:
		return "Http-Equiv";
	case META_PROPERTY:
		return "Property";
	default:
		/* Error */
		return NULL;
	}
}

int main(int argc, char **argv)
{
	struct ftml_html_output output;
	struct ftml_page_info page_info = {
		"my-page",
		NULL,
		"www",
		"Test page!",
		NULL,
		69.0,
		NULL,
		0,
		"C",
	};
	const char *input = (
		"[[css]]\n"
		"div.blockquote { color: blue; }\n"
		"[[/css]]\n"
		"**Test**\n"
		"[[module CSS]]\n"
		".my-class {\n"
		"    display: block;\n"
		"}\n"
		"[[/module]]\n"
		"__string__\n"
	);

	ftml_render_html(&output, input, &page_info);

	printf("Input:\n%s\n----\n\n", input);
	printf("Body:\n%s\n----\n\n", output.body);
	printf("Styles:\n");
	for (size_t i = 0; i < output.styles_len; i++) {
		printf("%s\n", output.styles_list[i]);

		if (i < output.styles_len - 1) {
			printf("----\n");
		} else {
			printf("\n\n");
		}
	}

	printf("Meta Fields:\n");
	for (size_t i = 0; i < output.meta_len; i++) {
		struct ftml_html_meta *meta = &output.meta_list[i];

		printf("    Type: %s\n", meta_type(meta->tag_type));
		printf("    Name: %s\n", meta->name);
		printf("    Value: %s\n", meta->value);

		if (i < output.meta_len - 1) {
			printf("    ----\n");
		} else {
			printf("\n\n");
		}
	}

	printf("Warnings:\n");
	for (size_t i = 0; i < output.warning_len; i++) {
		struct ftml_warning *warn = &output.warning_list[i];

		printf("    Token: %s\n", warn->token);
		printf("    Rule: %s\n", warn->rule);
		printf("    Span: %zu..%zu\n", warn->span_start, warn->span_end);
		printf("    Kind: %s\n", warn->kind);

		if (i < output.warning_len - 1) {
			printf("    ----\n");
		}
	}

	return 0;
}
