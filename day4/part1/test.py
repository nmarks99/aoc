matches = 10

score = [1]
for i in range(1,matches):
    score.append(score[i-1]*2)
print(score)


if matches > 0:
    score = 1
    for i in range(1, matches):
        score = score * 2
print(score)
