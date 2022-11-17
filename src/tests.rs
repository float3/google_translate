// Copyright (C) 2022  float3

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[test]
fn it_works() {
    println!("Translating \"test\" into german:");
    let text = "test";
    let source_language = super::lang::LanguageCode::de;
    let target_language = super::lang::LanguageCode::en;
    let result = super::translate(text, source_language, target_language);
    match result {
        Result::Ok(result) => {
            for res in result {
                println!("{}", res)
            }
        }
        _ => println!("failed"),
    }
}
