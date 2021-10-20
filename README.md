<p align="center">
  <br /><img
    width="75%"
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
  <img width="85%"　alt="droid with Hyper" src="https://rawcdn.githack.com/wasabeef/droid/7bc5f18646d3f9fd121cb5a5bde606207120fba4/media/demo.svg" >
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
   droid api <level>
   droid a <level>
   ```
  
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
   
   **"Search for words that start with `<number>`"**
   
   ```
   droid version <number>
   droid v <number>
   ```

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
   droid code <level>
   droid c <level>
   ```

   ```
   % droid code pie      


   +---------+-----------+-----------+--------------+
   | VERSION | CODE_NAME | API_LEVEL | RELEASE_DATE |
   +---------+-----------+-----------+--------------+
   | 9       | Pie       | 28        | 2018-08-06   |
   +---------+-----------+-----------+--------------+

   ```
   
   
   ### Find All
   
   ```
   droid list
   droid
   ```
   
   ```
   % droid

   +---------+----------------------------------+-----------+--------------+
   | VERSION | CODE_NAME                        | API_LEVEL | RELEASE_DATE |
   +---------+----------------------------------+-----------+--------------+
   | 1.0     | Alpha                            | 1         | 2008-09-23   |
   | 1.1     | Beta                             | 2         | 2009-02-09   |
   | 1.5     | Cupcake                          | 3         | 2009-04-27   |
   | 1.6     | Donut                            | 4         | 2009-09-15   |
   | 2.0     | Eclair                           | 5         | 2009-10-26   |
   | 2.0.1   | Eclair                           | 6         | 2009-12-03   |
   | 2.1     | Eclair                           | 7         | 2010-01-12   |
   | 2.2     | Froyo                            | 8         | 2010-03-20   |
   | 2.2.1   | Froyo                            | 8         | 2011-01-18   |
   | 2.2.2   | Froyo                            | 8         | 2011-01-22   |
   | 2.2.3   | Froyo                            | 8         | 2011-11-21   |
   | 2.3     | Gingerbread                      | 9         | 2010-12-06   |
   | 2.3.1   | Gingerbread                      | 9         | 2010-12      |
   | 2.3.2   | Gingerbread                      | 9         | 2011-01      |
   | 2.3.3   | Gingerbread                      | 10        | 2011-02-09   |
   | 2.3.4   | Gingerbread                      | 10        | 2011-04-28   |
   | 2.3.5   | Gingerbread                      | 10        | 2011-07-25   |
   | 2.3.6   | Gingerbread                      | 10        | 2011-09-02   |
   | 2.3.7   | Gingerbread                      | 10        | 2011-09-21   |
   | 3.0     | Honeycomb                        | 11        | 2011-02-22   |
   | 3.1     | Honeycomb                        | 12        | 2011-03-10   |
   | 3.2     | Honeycomb                        | 13        | 2011-07-15   |
   | 3.2.1   | Honeycomb                        | 13        | 2011-11-20   |
   | 3.2.2   | Honeycomb                        | 13        | 2011-08-30   |
   | 3.2.3   | Honeycomb                        | 13        | 2011-08-30   |
   | 3.2.4   | Honeycomb                        | 13        | 2011-12      |
   | 3.2.5   | Honeycomb                        | 13        | 2012-01      |
   | 3.2.6   | Honeycomb                        | 13        | 2012-12      |
   | 4.0     | Ice Cream Sandwich               | 14        | 2011-10-18   |
   | 4.0.1   | Ice Cream Sandwich               | 14        | 2011-10-21   |
   | 4.0.2   | Ice Cream Sandwich               | 14        | 2011-11-28   |
   | 4.0.3   | Ice Cream Sandwich               | 15        | 2011-12-16   |
   | 4.0.4   | Ice Cream Sandwich               | 15        | 2012-03-29   |
   | 4.1     | Jelly Bean                       | 16        | 2012-07-09   |
   | 4.1.1   | Jelly Bean                       | 16        | 2012-07-11   |
   | 4.1.2   | Jelly Bean                       | 16        | 2012-10-09   |
   | 4.2     | Jelly Bean                       | 17        | 2012-11-13   |
   | 4.2.1   | Jelly Bean                       | 17        | 2012-11-27   |
   | 4.2.2   | Jelly Bean                       | 17        | 2013-02-11   |
   | 4.3     | Jelly Bean                       | 18        | 2013-07-24   |
   | 4.3.1   | Jelly Bean                       | 18        | 2013-10-03   |
   | 4.4     | KitKat                           | 19        | 2013-10-31   |
   | 4.4.1   | KitKat                           | 19        | 2013-12-05   |
   | 4.4.2   | KitKat                           | 19        | 2013-12-09   |
   | 4.4.3   | KitKat                           | 19        | 2014-06-02   |
   | 4.4.4   | KitKat                           | 19        | 2014-06-19   |
   | 4.4W    | KitKat, with wearable extensions | 20        | 2014-06-25   |
   | 4.4W.1  | KitKat, with wearable extensions | 20        | 2014-09-06   |
   | 4.4W.2  | KitKat, with wearable extensions | 20        | 2014-10-21   |
   | 5.0     | Lollipop                         | 21        | 2014-11-12   |
   | 5.0.1   | Lollipop                         | 21        | 2014-12-02   |
   | 5.0.2   | Lollipop                         | 21        | 2014-12-19   |
   | 5.1     | Lollipop                         | 22        | 2015-03-09   |
   | 5.1.1   | Lollipop                         | 22        | 2015-04-21   |
   | 6.0     | Marshmallow                      | 23        | 2015-10-05   |
   | 6.0.1   | Marshmallow                      | 23        | 2015-12-07   |
   | 7.0     | Nougat                           | 24        | 2016-08-22   |
   | 7.1     | Nougat                           | 25        | 2016-10-04   |
   | 7.1.1   | Nougat                           | 25        | 2016-12-05   |
   | 7.1.2   | Nougat                           | 15        | 2017-04-04   |
   | 8.0     | Oreo                             | 26        | 2017-08-21   |
   | 8.1.0   | Oreo                             | 27        | 2017-12-05   |
   | 9       | Pie                              | 28        | 2018-08-06   |
   | 10      | Q                                | 29        | 2019-09-03   |
   | 11      | R                                | 30        | 2020-09-08   |
   | 12      | S                                | 31        | 2021-10-20   |
   +---------+----------------------------------+-----------+--------------+

   ```
   
<br />
<br />
<br />
<br />

 (*・ﾉｪ・) Recently, I'm studying English by watching the Queer Eye on Netflix.
