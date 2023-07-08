void setup() {
  Serial.begin(9600);

}
int potVal = 0;

void loop() {
    potVal = analogRead(A1);
    int faderValue = map(potVal, 0, 1023, 0, 100);
    Serial.println(faderValue);
    delay(50);
}
