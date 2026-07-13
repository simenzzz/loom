FROM python:3.12-slim
WORKDIR /app
COPY pyproject.toml ./
COPY src ./src
RUN pip install --no-cache-dir .
RUN useradd --system --uid 10001 loom
USER loom
EXPOSE 7720
CMD ["uvicorn", "src.main:app", "--host", "0.0.0.0", "--port", "7720"]
