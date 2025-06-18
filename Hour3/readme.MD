(1) Demi keamanan, nilai dari sebuah variabel yang diikat (bound variable) bersifat tidak dapat diubah (immutable) secara default.

rust
Salin
Edit
let (var1, var2) = (value1, value2);
Setelah sebuah variabel diikat, maka var1 sama dengan value1, dan var2 sama dengan value2.
Nilai-nilai tersebut tidak dapat diubah.

Jika kamu ingin membuat variabel yang bisa diubah, kamu bisa menggunakan mut seperti ini:

rust
Salin
Edit
let mut x = 10;
x = 20; // ini diperbolehkan karena x adalah mutable