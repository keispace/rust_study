https://doc.rust-lang.org/cargo/index.html

종속성 관리 도구(like npm)
공식 저장소: creates.io (https://www.npmjs.com)

test, 벤치마크 가능. (cargo test --- npm test)
- npm과 달리 커맨드 스크립트 커스텀 X
설정파일 
Cargo.toml --- package.json
Cargo.lock --- package-lock.json 

자주 쓰는 커맨드
cargo new {패키지명} --bin
- bin은 디폴트로 생략가능. 바이너리 프로젝트 생성
cargo new {라이브러리명} --lib
- 라이브러리 프로젝트 생성
cargo run
- 디버그 빌드 후 실행
cargo build -r 
- 릴리즈(-r. 생략시 디버그) 버전 빌드
cargo test 
- test 어트리뷰트, 문서주석 테스트. 
cargo doc --open
- 문서화 페이지 작성 및 오픈. (swagger)