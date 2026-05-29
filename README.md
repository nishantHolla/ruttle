# ruttle

Ruttle is a fast static templating engine written in Rust for generating HTML, CSS, and other text-based assets.
It is designed for building static websites with minimal runtime overhead. It introduces a lightweight template language focused on:

- Variable definition and interpolation
- File composition through includes
- Conditional rendering
- Iteration
- JSON and Markdown integration
- Compile-time deduplication

Templates are evaluated at build time and compiled into plain static output.

## Installation

Install using Cargo:

```bash
cargo install ruttle
```

Verify the installation:

```bash
ruttle --version
```

## Usage

Compile one or more template files into an output directory:

```bash
ruttle --output ./dist ./src/index.part.html
```

Compile multiple files:

```bash
ruttle --output ./dist ./src/index.part.html ./src/about.part.html
```

Compile and minify output:

```bash
ruttle --minify --output ./dist ./src/index.part.html
```

Compile with debug logging enabled:

```bash
ruttle --debug --output ./dist ./src/index.part.html
```

## Command Line Options

| Flag | Description |
|------|-------------|
| `-o`, `--output <DIR>` | Output directory |
| `-m`, `--minify` | Minify generated output |
| `-d`, `--debug` | Enable debug logging |

### Arguments

- `<INPUTS>...`: One or more template files to compile. Must end with `.part.html` to prevent accidental overwriting
if the input and output directories are the same.

## Example

Given the file:

```html
<!-- src/index.rtl -->
{#define title="Hello from Ruttle"}

<h1>{#value title}</h1>
```

Run:

```bash
ruttle --output ./dist ./src/index.part.html
```

Generated output:

```html
<h1>Hello from Ruttle</h1>
```

## Template Syntax

- [Variable Definition and Interpolation](#variable-definition-and-interpolation)
- [File Inclusion with Props](#file-inclusion-with-props)
- [Conditional Rendering](#conditional-rendering)
- [Numeric For Loops](#numeric-for-loops)
- [JSON Iteration](#json-iteration)
- [Working with JSON Files](#working-with-json-files)
- [Working with Markdown Files](#working-with-markdown-files)
- [Single Inclusion](#single-inclusion)

### Variable Definition and Interpolation

Variables can be defined using the `#define` directive and can be interpolated using the `#value` directive

```html
{#define title="Hello, world"}

<h1>{#value title}</h1>
```

this compiles to

```html
<h1>Hello, world</h1>
```

### File Inclusion with Props

Other files can be included using the `#include` directives that takes in a list of key-value pairs as
properties.

If a file called `button.html` exists with this content
```html
<button class="{#value color}" style="opacity:{#value opacity}">
    Click me
</button>
```

It can be included multiple times by doing this
```html
{#include ./button.html color="red" opacity="0.8"}
```

this compiles to

```html
<button class="red" style="opacity:0.8">
    Click me
</button>
```

This enables reusable components with customizable inputs.

### Conditional Rendering

Conditional rendering can be achieved using the `#if`, `#elseif` and `#else` directive

```html
{#define role="admin"}

{#if role=="admin"
    <h1>Admin Panel</h1>
#elseif role=="user"
    <h1>User Dashboard</h1>
#else
}
```

this comiles to

```html
<h1>Admin Panel</h1>
```

The supported operations are:
- less than (`<`)
- greater than (`>`)
- less than or equal to (`>=`)
- greater than or equal to (`<=`)
- equal to (`==`)
- not equal to (`!=`)

Values are automatically casted to integers or doubles if required

### Numeric For Loops

Iterate over a numeric range using the `#for` directive

```html
{#for i, value in 1..5.1
    <li>{#value value}</li>
}
```

this compiles to

```html
<li>1</li>
<li>2</li>
<li>3</li>
<li>4</li>
```

the syntax for defining a range is `start..end..step` where `start` is inclusive and `end` is exclusive.

### JSON iteration

Loop through arrays or objects from JSON files using the `#for` directive.

If `./data.json` is a json file with the following content
```json
{
  "a": "Apple",
  "b": "Banana"
}
```

the object can be iterated over by

```html
{#for key, value in ./data.json
    <p>{#value key}: {#value value}</p>
}
```

this compiles to

```html
<p>a: Apple</p>
<p>b: Banana</p>
```

this works with JSON Objects and JSON arrays

### Working with JSON Files

Values in json files can be accessed using the `with` directive

If `./data.json` is a json file with the following content
```json
{
    "title": "My post",
    "description": "Post description"
}
```

it can be rendered using
```html
{#with file as ./data.json
    <h1>{#value file.title}</h1>
    <p>{#value file.description}</p>
}
```

this compiles to
```html
<h1>My post</h1>
<p>Post description</p>
```

### Working with Markdown Files

Markdown files can be rendered as html using the `with` directive

If `./content.md` is a markdown file with the following content
```md
---
title: My Post
author: John
---

# Hello World

This is markdown content.
```

it can be rendered using
```html
{#with file as ./post.md
    <article>
        <h1>{#value file.title}</h1>
        {#value file.content}
    </article>
}
```

this compiles to

```html
<article>
    <h1>My Post</h1>
    <h1>Hello World</h1>
    <p>This is markdown content.</p>
</article>
```

### Single Inclusion

The `#once` directive ensures content is emitted only once, even if encountered multiple times.

If `./component.html` is defined like this
```html
{#once <style>

.button {
    color: red;
}

</style> }

<button class="button">Click</button>
```

and is used like this

```html
{#include ./component.html}
{#include ./component.html}
```

this compiles to

```html
<style>
.button {
    color: red;
}
</style>

<button class="button">Click</button>
<button class="button">Click</button>
```

This can be used for:

- Shared styles
- Script injection
- Preventing duplicate assets
