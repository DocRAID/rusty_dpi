# (중요) 아직 아무기능 없음

# Rusty DPI
네트워크 공부겸 DPI (심층 패킷 분석) 우회 툴

## 수동적 DPI

1. (X) 리다이렉션 패킷 차단

## 능동적 DPI 

1. (X) 첫 데이터 패킷을 TCP 단에서 단편화 (내용 → 내(flush)용)
2. (X) keep-alive 패킷을 TCP 단에서 단편화 (Connection: keep-alive 상태에서 우회 무한 루프 반복 적용)
3. (X) HTTP Host 헤더를 hoSt로 교체 (host: test.test → hoSt: test.test)
4. (inProgress) HTTP Host 헤더 이름과 값 사이의 공백 제거 (host: test.test → host:test.test)
5. (X) HTTP 메소드(GET, POST 등)와 URI 사이에 공백 추가 (GET / HTTP1.1 → GET　/ HTTP1.1)
6. (X) HTTP Host 헤더 값에 대소문자 섞기 (test.com → tEsT.cOm)

---
Reference
[GoodbyeDPI](https://github.com/ValdikSS/GoodbyeDPI)
