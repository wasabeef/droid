<p align="center">
  <br /><img
    width="400"
    src="https://raw.githubusercontent.com/wasabeef/droid/master/media/logo.png"
    alt="droid – Logo"/>
</p>
<p align="center">
  <a href="https://github.com/wasabeef/droid/actions">
    <img
      src="https://github.com/wasabeef/droid/workflows/Continuous%20Integration/badge.svg"
      alt="GitHub Actions workflow status"/>
  </a>
  <a href="https://crates.io/crates/droid">
    <img src="https://badgen.net/crates/v/droid" alt="Crates.io version"/>
  </a>
  <br />
</p>

<h4 align="center">
  <br />
  <a href="#installation">Installation</a>
  -
  <a href="#usage">Usage</a>
</h4>

<h1></h1>

<p align="center">A command-line tool for checking Android OS version history written by Rust.<p>

<p align="center">
  <br>
  <img alt="droid with Hyper" src="https://raw.githubusercontent.com/wasabeef/droid/master/media/demo.gif" width="80%">
  <br>
  <br>
</p>

## Installation

1. Install the **droid** binary:

   ### Homebrew

   ```sh
   % brew install wasabeef/tap/droid
   ```

   ### Rust

   ```sh
   % cargo install droid
   ```
  
  **[Download archives of precompiled binaries](https://github.com/wasabeef/droid/releases)** if you don't use the platforms below. 

## Usage

   ### Find by API Level

   ```
   % droid api 21      


   +---------+-----------+-----------+--------------+
   | VERSION | CODE_NAME | API_LEVEL | RELEASE_DATE |
   +---------+-----------+-----------+--------------+
   | 5.0     | Lollipop  | 21        | 2014-11-12   |
   | 5.0.1   | Lollipop  | 21        | 2014-12-02   |
   | 5.0.2   | Lollipop  | 21        | 2014-12-19   |
   +---------+-----------+-----------+--------------+

   ```
   
   ### Find by Version number

   ```
   % droid version 4.4


   +---------+----------------------------------+-----------+--------------+
   | VERSION | CODE_NAME                        | API_LEVEL | RELEASE_DATE |
   +---------+----------------------------------+-----------+--------------+
   | 4.4     | KitKat                           | 19        | 2013-10-31   |
   | 4.4.1   | KitKat                           | 19        | 2013-12-05   |
   | 4.4.2   | KitKat                           | 19        | 2013-12-09   |
   | 4.4.3   | KitKat                           | 19        | 2014-06-02   |
   | 4.4.4   | KitKat                           | 19        | 2014-06-19   |
   | 4.4W    | KitKat, with wearable extensions | 20        | 2014-06-25   |
   | 4.4W.1  | KitKat, with wearable extensions | 20        | 2014-09-06   |
   | 4.4W.2  | KitKat, with wearable extensions | 20        | 2014-10-21   |
   +---------+----------------------------------+-----------+--------------+

   ```
   
   ### Find by Code name

   ```
   % droid code pie      


   +---------+-----------+-----------+--------------+
   | VERSION | CODE_NAME | API_LEVEL | RELEASE_DATE |
   +---------+-----------+-----------+--------------+
   | 9       | Pie       | 28        | 2018-08-06   |
   +---------+-----------+-----------+--------------+

   ```
