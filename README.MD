# **image-modifier**

![Build Status](https://img.shields.io/github/workflow/status/c1m50c/image-modifier/Build?style=for-the-badge)
![Crates.io Version](https://img.shields.io/crates/v/imgmod?color=orange&style=for-the-badge)
![Crates.io License](https://img.shields.io/crates/l/imgmod?style=for-the-badge)
![Lines of Code](https://img.shields.io/tokei/lines/github/c1m50c/image-modifier?style=for-the-badge)
![GitHub Stars](https://img.shields.io/github/stars/c1m50c/image-modifier?color=yellow&style=for-the-badge)

Command line tool to simply modify image files with various filters.


---


## **Installing**
You can directly install the binary exectuable from <a href="https://crates.io/crates/imgmod">crates.io</a> with <a href="https://doc.rust-lang.org/cargo/">Cargo</a> by running the following command.

```bash
$ cargo install imgmod
```

## **Running**
After installation running is just as easy, run `imgmod` in your command line with the following argument pattern.

```bash
$ imgmod [MODIFIER] [INPUT_IMAGE_PATH] [OUTPUT_IMAGE_PATH]
```


---


## **Modifiers**
<details>
<summary>Greyscale</summary>
Converts the input image into its greyscaled counterpart.

**Valid Modifier Names**
- greyscale
- grayscale
- gs
</details>

<details>
<summary>Invert</summary>
Inverses all the colors of the input image.

**Valid Modifier Names**
- invert
- inverse
- inv
- i
</details>

<details>
<summary>Red</summary>
Isolates the colors to only the red channel.

**Valid Modifier Names**
- red
- r
</details>

<details>
<summary>Green</summary>
Isolates the colors to only the green channel.

**Valid Modifier Names**
- green
- g
</details>

<details>
<summary>Blue</summary>
Isolates the colors to only the blue channel.

**Valid Modifier Names**
- blue
- b
</details>

<details>
<summary>Alpha</summary>
Converts the value of all color channels to the value of their alpha channel within the input image.

**Valid Modifier Names**
- alpha
- a
</details>

<details>
<summary>Blur</summary>
Blurs the image by a fixed amount.

**Valid Modifier Names**
- blur
</details>